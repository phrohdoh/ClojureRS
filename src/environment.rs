use crate::clojure_std;
use crate::clojure_string;
use crate::error_message;
use crate::namespace::Namespaces;
use crate::repl::Repl;
use crate::rust_core;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! insert_into_ns {
    ($env:expr,$ns:literal,$($sym:literal,$val:expr),+) => {
        $(
            $crate::environment::Environment::insert_into_namespace(
                $env,
                &$crate::symbol::Symbol::intern($ns),
                $crate::symbol::Symbol::intern($sym),
                $crate::value::ToValue::to_rc_value($val),
            );
        )*
    };
}


// @TODO lookup naming convention
/// Inner value of our environment
/// See Environment for overall purpose
#[derive(Debug, Clone)]
pub struct EnvironmentVal {
    //@TODO is it worth just making this a mutable reference (to an
    // immutable value), and referencing the current symbol at any
    // point in time?  Is implementing that sort of speedup in general
    // significant
    curr_ns_sym: RefCell<Symbol>,
    namespaces: Namespaces,
}
impl EnvironmentVal {
    // @TODO is this wrapper really necessary, or is it just inviting an invariant break?
    /// Note; do not use. Does not enforce the invariant that namespace exist
    /// Use change_or_create_namespace instead
    fn change_namespace(&self, name: Symbol) {
        self.curr_ns_sym.replace(name);
    }
    fn change_or_create_namespace(&self, symbol: &Symbol) {
        if self.has_namespace(symbol) {
            self.change_namespace(symbol.unqualified());
        } else {
            self.create_namespace(symbol);
            self.change_namespace(symbol.unqualified());
        }
    }
    fn add_referred_syms(&self, namespace_sym: &Symbol, syms: HashMap<Symbol, Vec<Symbol>>) {
        self.namespaces.add_referred_syms(namespace_sym, syms);
    }
    fn add_referred_namespace(&self, namespace_sym: &Symbol, referred_namespace_sym: &Symbol) {
        self.namespaces
            .add_referred_namespace(namespace_sym, referred_namespace_sym);
    }
    fn insert_into_namespace(&self, namespace_sym: &Symbol, sym: Symbol, val: Rc<Value>) {
        self.namespaces
            .insert_into_namespace(namespace_sym, &sym, val);
    }
    fn insert_into_current_namespace(&self, sym: Symbol, val: Rc<Value>) {
        self.namespaces
            .insert_into_namespace(&*self.curr_ns_sym.borrow(), &sym, val);
    }
    fn has_namespace(&self, namespace: &Symbol) -> bool {
        self.namespaces.has_namespace(namespace)
    }
    fn get_var_from_namespace(&self, namespace: &Symbol, sym: &Symbol) -> Rc<Value> {
        self.namespaces.get_var(namespace, sym)
    }
    fn get_from_namespace(&self, namespace: &Symbol, sym: &Symbol) -> Rc<Value> {
        self.namespaces.get(namespace, sym)
    }
    fn get_current_namespace(&self) -> Symbol {
        self.curr_ns_sym.borrow().clone()
    }

    fn create_namespace(&self, symbol: &Symbol) {
        self.namespaces.create_namespace(symbol);
    }
    // @TODO as mentioned, we've been working with a memory model where values exist
    //       in our system once-ish and we reference them all over with Rc<..>
    //       Look into possibly working this into that (if its even significant);
    /// Default main environment
    fn new_main_val() -> EnvironmentVal {
        let curr_ns_sym = Symbol::intern("user");
        let namespaces = Namespaces::new();
        namespaces.create_namespace(&curr_ns_sym);
        EnvironmentVal {
            curr_ns_sym: RefCell::new(curr_ns_sym),
            namespaces,
        }
    }
}
/// Our environment keeps track of the meaning of things 'right here', relative to where
/// something is at (meaning, a form inside of a let might have a different meaning for
/// the symbol x than a form outside of it, with a let introducing an additional local environment
///
/// Stores our namespaces and our current namespace, which themselves personally store our symbols
/// mapped to values
#[derive(Debug, Clone)]
pub enum Environment {
    MainEnvironment(EnvironmentVal),
    /// Points to parent environment
    /// Introduced by Closures, and by let
    LocalEnvironment(Rc<Environment>, RefCell<HashMap<Symbol, Rc<Value>>>),
}
use Environment::*;
impl Environment {
    pub fn has_namespace(&self, symbol: &Symbol) -> bool {
        match self.get_main_environment() {
            MainEnvironment(env_val) => env_val.has_namespace(symbol),
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		             but by definition should only return MainEnvironment"
            ),
        }
    }
    pub fn add_referred_syms(&self, namespace_sym: &Symbol, syms: HashMap<Symbol, Vec<Symbol>>) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => {
                env_val.add_referred_syms(namespace_sym, syms);
            }
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		             but by definition should only return MainEnvironment"
            ),
        }
    }
    pub fn add_referred_syms_to_curr_namespace(&self, syms: HashMap<Symbol, Vec<Symbol>>) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => {
                let namespace_sym = self.get_current_namespace();
                env_val.add_referred_syms(&namespace_sym, syms);
            }
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		             but by definition should only return MainEnvironment"
            ),
        }
    }
    pub fn add_referred_namespace_to_curr_namespace(&self, referred_namespace_sym: &Symbol) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => {
                let namespace_sym = self.get_current_namespace();
                env_val.add_referred_namespace(&namespace_sym, referred_namespace_sym);
            }
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		             but by definition should only return MainEnvironment"
            ),
        }
    }
    /// Changes the current namespace, or creates one first if
    /// namespace doesn't already exist
    pub fn change_or_create_namespace(&self, symbol: &Symbol) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => {
                env_val.change_or_create_namespace(symbol);
            }
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		             but by definition should only return MainEnvironment"
            ),
        }
    }
    // @TODO consider 'get_current_..' for consistency?
    // @TODO consider 'current_namespace_sym'? after all, its not the namespace itself
    pub fn get_current_namespace(&self) -> Symbol {
        match self.get_main_environment() {
            MainEnvironment(EnvironmentVal { curr_ns_sym, .. }) => curr_ns_sym.borrow().clone(),
            LocalEnvironment(..) => panic!(
                "In get_current_namespace_name(): get_main_environment() returns LocalEnvironment,\
		                 but by definition should only return MainEnvironment"
            ),
        }
    }
    // Note; since we're now dealing with curr_ns as a refcell, we're
    // returning a String instead of a &str, as I suspect a &str could
    // risk becoming invalid as curr_ns changes
    pub fn get_current_namespace_name(&self) -> String {
        self.get_current_namespace().name.clone()
    }

    pub fn new_main_environment() -> Environment {
        MainEnvironment(EnvironmentVal::new_main_val())
    }
    pub fn new_local_environment(outer_environment: Rc<Environment>) -> Environment {
        LocalEnvironment(outer_environment, RefCell::new(HashMap::new()))
    }
    /// Insert a binding into an arbitrary namespace
    pub fn insert_into_namespace(&self, namespace: &Symbol, sym: Symbol, val: Rc<Value>) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => env_val.insert_into_namespace(namespace, sym, val),
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		                 but by definition should only return MainEnvironment"
            ),
        }
    }
    pub fn insert_into_current_namespace(&self, sym: Symbol, val: Rc<Value>) {
        match self.get_main_environment() {
            MainEnvironment(env_val) => env_val.insert_into_current_namespace(sym, val),
            LocalEnvironment(..) => panic!(
                "get_main_environment() returns LocalEnvironment,\
		                 but by definition should only return MainEnvironment"
            ),
        }
    }
    /// Insert into the environment around you;  the local bindings,
    /// or the current namespace, if this is top level
    /// For instance,
    /// ```clojure
    ///   (def a 1)      ;; => main_environment.insert(a,1)
    ///   (let [a 1] ..) ;; => local_environment.insert(a,1)  
    pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        match self {
            MainEnvironment(_) => {
                self.insert_into_current_namespace(sym, val);
            }
            LocalEnvironment(_, mappings) => {
                mappings.borrow_mut().insert(sym, val);
            }
        }
    }
    fn get_main_environment(&self) -> &Self {
        match self {
            MainEnvironment(_) => self,
            LocalEnvironment(parent_env, ..) => parent_env.get_main_environment(),
        }
    }
    pub fn get_var(&self, sym: &Symbol) -> Rc<Value> {
        match self {
            MainEnvironment(env_val) => {
                // If we've recieved a qualified symbol like
                // clojure.core/+
                match sym.namespace() {
                    Some(ns) => env_val.get_var_from_namespace(&Symbol::intern(ns), sym),
                    _ => env_val.get_var_from_namespace(
                            &env_val.get_current_namespace(),
                            &Symbol::intern(&sym.name),
                        ),
                }
            }
            LocalEnvironment(parent_env, mappings) => {
                match sym.namespace() {
                    Some(_ns) => self.get_main_environment().get(sym),
                    _ => match mappings.borrow().get(sym) {
                        Some(val) => Rc::clone(val),
                        _ => parent_env.get(sym),
                    }
                }
            }
        }
    }
    // @TODO refactor to use ^
    // @TODO figure out convention for 'ns' vs 'namespace'
    /// Get closest value "around" us;  try our local environment, then
    /// try our main environment (unless its namespace qualified)
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self {
            MainEnvironment(env_val) => {
                // If we've recieved a qualified symbol like
                // clojure.core/+
                match sym.namespace() {
                    Some(ns) => env_val.get_from_namespace(&Symbol::intern(ns), sym), // Use that namespace
                    _ => env_val.get_from_namespace(&env_val.get_current_namespace(), &sym),
                }
            }
            LocalEnvironment(parent_env, mappings) => {
                if sym.has_ns() {
                    self.get_main_environment().get(sym)
                } else {
                    match mappings.borrow().get(sym) {
                        Some(val) => Rc::clone(val),
                        _ => parent_env.get(sym),
                    }
                }
            }
        }
    }

    pub fn populate_with_clojure_core(env: Rc<Environment>) {
        // Register our macros / functions ahead of time
        let add_fn = rust_core::AddFn {};
        let subtract_fn = rust_core::SubtractFn {};
        let multiply_fn = rust_core::MultiplyFn {};
        let divide_fn = rust_core::DivideFn {};
        let rem_fn = rust_core::RemFn {};
        let rand_fn = rust_core::RandFn {};
        let rand_int_fn = rust_core::RandIntFn {};
        let str_fn = rust_core::StrFn {};
        let do_fn = rust_core::DoFn {};
        let nth_fn = rust_core::NthFn {};
        let do_macro = rust_core::DoMacro {};
        let concat_fn = rust_core::ConcatFn {};
        let flush_stdout_fn = rust_core::FlushStdoutFn {};
        let system_newline_fn = rust_core::SystemNewlineFn {};
        let print_string_fn = rust_core::PrintStringFn {};
        let read_line_fn = rust_core::ReadLineFn {};
        let assoc_fn = rust_core::AssocFn {};
        let more_fn = rust_core::MoreFn {};
        let first_fn = rust_core::FirstFn {};
        let second_fn = rust_core::SecondFn {};

        // rust implementations of core functions
        let slurp_fn = rust_core::slurp::SlurpFn {};

        // clojure.std functions
        let thread_sleep_fn = clojure_std::thread::SleepFn {};
        let nanotime_fn = clojure_std::time::NanoTimeFn {};
        let get_env_fn = clojure_std::env::GetEnvFn {};

        let get_fn = rust_core::GetFn {};
        let map_fn = rust_core::MapFn {};

        // clojure.string
        let reverse_fn = clojure_string::reverse::ReverseFn {};
        let join_fn = clojure_string::join::JoinFn {};
        let blank_fn = clojure_string::blank_qmark_::BlankFn {};
        let upper_case_fn = clojure_string::upper_case::UpperCaseFn {};
        let lower_case_fn = clojure_string::lower_case::LowerCaseFn {};
        let starts_with_fn = clojure_string::starts_with_qmark_::StartsWithFn {};
        let ends_with_fn = clojure_string::ends_with_qmark_::EndsWithFn {};
        let includes_fn = clojure_string::includes_qmark_::IncludesFn {};
        let trim_fn = clojure_string::trim::TrimFn {};
        let triml_fn = clojure_string::triml::TrimLFn {};
        let trimr_fn = clojure_string::trimr::TrimRFn {};
        let trim_newline_fn = clojure_string::trim_newline::TrimNewlineFn {};
        let split_fn = clojure_string::split::SplitFn {};

        // Hardcoded fns
        let lexical_eval_fn = Value::LexicalEvalFn {};
        // Hardcoded macros
        let let_macro = Value::LetMacro {};
        let quote_macro = Value::QuoteMacro {};
        let def_macro = Value::DefMacro {};
        let fn_macro = Value::FnMacro {};
        let defmacro_macro = Value::DefmacroMacro {};
        let if_macro = Value::IfMacro {};

        let equals_fn = rust_core::EqualsFn {};
        let type_fn = rust_core::TypeFn {};
        let eval_fn = rust_core::EvalFn::new(Rc::clone(&env));
        let ns_macro = rust_core::NsMacro::new(Rc::clone(&env));
        let load_file_fn = rust_core::LoadFileFn::new(Rc::clone(&env));
        let refer_fn = rust_core::ReferFn::new(Rc::clone(&env));
        let meta_fn = rust_core::MetaFn::new(Rc::clone(&env));
        let with_meta_fn = rust_core::WithMetaFn::new(Rc::clone(&env));
        let var_fn = rust_core::special_form::VarFn::new(Rc::clone(&env));
        let count_fn = rust_core::count::CountFn {};
        let lt_fn = rust_core::lt::LtFn {};
        let gt_fn = rust_core::gt::GtFn {};
        let lte_fn = rust_core::lte::LteFn {};
        let gte_fn = rust_core::gte::GteFn {};

        env.change_or_create_namespace(&Symbol::intern("clojure.core"));

        insert_into_ns!(&env, "clojure.core",
            "+",            &add_fn,
            "-",            &subtract_fn,
            "*",            &multiply_fn,
            "/",            &divide_fn,
            "rem",          &rem_fn,
            "rand",         &rand_fn,
            "rand-int",     &rand_int_fn,
            "let",          &let_macro,
            "str",          &str_fn,
            "quote",        &quote_macro,
            "def",          &def_macro,
            "fn",           &fn_macro,
            "defmacro",     &defmacro_macro,
            "eval",         &eval_fn,
            "meta",         &meta_fn,
            "with-meta",    &with_meta_fn,
            "var-fn*",      &var_fn,
            "count",        &count_fn,
            "quote",        &quote_macro,
            "do-fn*",       &do_fn,
            "do",           &do_macro,
            "def",          &def_macro,
            "if",           &if_macro,
            "ns",           &ns_macro,
            "lexical-eval", &lexical_eval_fn,
            "load-file",    &load_file_fn,
            "nth",          &nth_fn,
            "assoc",        &assoc_fn,
            "get",          &get_fn,
            "map",          &map_fn,
            "concat",       &concat_fn,
            "more",         &more_fn,
            "first",        &first_fn,
            "second",       &second_fn,
            "=",            &equals_fn,
            "type",         &type_fn,
            "refer",        &refer_fn,
            // input and output
            "system-newline", &system_newline_fn,
            "flush-stdout",   &flush_stdout_fn,
            "print-string",   &print_string_fn,
            "read-line",      &read_line_fn,
            // core.clj wraps calls to the rust implementations
            // @TODO add this to clojure.rs.core namespace as clojure.rs.core/slurp
            "rust-slurp", &slurp_fn,
            // interop to read real clojure.core
            "lt",  &lt_fn,
            "gt",  &gt_fn,
            "lte", &lte_fn,
            "gte", &gte_fn
        );

        insert_into_ns!(&env, "Thread",
            "sleep", &thread_sleep_fn);

        insert_into_ns!(&env, "System",
            "nanoTime", &nanotime_fn,
            "getenv", &get_env_fn
        );

        insert_into_ns!(&env, "clojure.string",
            "reverse",      &reverse_fn,
            "join",         &join_fn,
            "blank?",       &blank_fn,
            "upper-case",   &upper_case_fn,
            "lower-case",   &lower_case_fn,
            "starts-with?", &starts_with_fn,
            "ends-with?",   &ends_with_fn,
            "includes?",    &includes_fn,
            "trim",         &trim_fn,
            "triml",        &triml_fn,
            "trimr",        &trimr_fn,
            "trim-newline", &trim_newline_fn,
            "split",        &split_fn
        );

        //
        // Read in clojure.core
        //
        // @TODO its time for a RT (runtime), which environment seems to be becoming
        let _ = Repl::new(Rc::clone(&env)).try_eval_file("./src/clojure/core.clj");
        // TODO: should read into namespace if (ns ..) is given in source file
        let _ = Repl::new(Rc::clone(&env)).try_eval_file("./src/clojure/string.clj");
    }

    pub fn clojure_core_environment() -> Rc<Environment> {
        let env = Rc::new(Environment::new_main_environment());
        Environment::populate_with_clojure_core(env.clone());

        // We can add this back once we have requires
        // environment.change_or_create_namespace(Symbol::intern("user"));

        env
    }
}

#[cfg(test)]
mod tests {
    mod environment_val {
        use crate::environment::EnvironmentVal;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use std::rc::Rc;

        ////////////////////////////////////////////////////////////////////////////////
        //
        // pub fn get_current_namespace(&self) -> Symbol {
        //
        ////////////////////////////////////////////////////////////////////////////////

        #[test]
        fn get_current_namespace() {
            let env_val = EnvironmentVal::new_main_val();

            assert_eq!(Symbol::intern("user"), env_val.get_current_namespace());

            env_val.change_or_create_namespace(&Symbol::intern("core"));
            assert_eq!(Symbol::intern("core"), env_val.get_current_namespace());

            // @TODO add this invariant back next, and remove this comment; 5.9.2020
            // env_val.change_or_create_namespace(Symbol::intern_with_ns("not-ns","ns"));
            // assert_eq!(Symbol::intern("ns"),env_val.get_current_namespace())

            // @TODO add case for local environment
        }

        /////////////////////////////////////////////////////////////////////////////
        //
        //  fn get_from_namespace(&self,namespace: &Symbol,sym: &Symbol) -> Rc<Value>
        //
        //////////////////////////////////////////////////////////////////////////////

        #[test]
        fn get_from_namespace() {
            let env_val = EnvironmentVal::new_main_val();

            env_val.insert_into_namespace(
                &Symbol::intern("core"),
                Symbol::intern("+"),
                Rc::new(Value::Nil),
            );
            env_val.insert_into_namespace(
                &Symbol::intern_with_ns("dragon", "core"),
                Symbol::intern("+2"),
                Rc::new(Value::Nil),
            );
            env_val.insert_into_namespace(
                &Symbol::intern_with_ns("dragon", "core"),
                Symbol::intern_with_ns("override", "+3"),
                Rc::new(Value::Nil),
            );

            assert_eq!(
                Rc::new(Value::Nil),
                env_val.get_from_namespace(&Symbol::intern("core"), &Symbol::intern("+"))
            );

            assert_eq!(
                Rc::new(Value::Nil),
                env_val.get_from_namespace(&Symbol::intern("core"), &Symbol::intern("+2"))
            );

            assert_eq!(
                Rc::new(Value::Nil),
                env_val.get_from_namespace(&Symbol::intern("override"), &Symbol::intern("+3"))
            );
        }
    }
    mod environment {
        use crate::environment::Environment;
        use crate::environment::Environment::*;
        use crate::environment::EnvironmentVal;
        use crate::ifn::IFn;
        use crate::rust_core;
        use crate::symbol::Symbol;
        use crate::value::{ToValue, Value};
        use std::rc::Rc;
        ////////////////////////////////////////////////////////////////////////
        //
        // pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        //
        ////////////////////////////////////////////////////////////////////////
        #[test]
        fn get_plus() {
            let add_fn = rust_core::AddFn {};

            let environment = Rc::new(Environment::new_main_environment());
            environment.insert(Symbol::intern("+"), add_fn.to_rc_value());

            let plus = environment.get(&Symbol::intern("+"));

            assert_eq!(
                8.to_value(),
                add_fn.invoke(vec![3_i32.to_rc_value(), 5_i32.to_rc_value()])
            );

            if let Value::IFn(add_ifn) = &*plus {
                assert_eq!(
                    8.to_value(),
                    add_ifn.invoke(vec![3_i32.to_rc_value(), 5_i32.to_rc_value()])
                );
                return;
            }
            panic!("get_plus: plus is: {:#?}", plus);
        }
        /////////////////////////////////////////////////////////////////////////
        //
        // pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        //
        /////////////////////////////////////////////////////////////////////////
        #[test]
        fn insert_plus() {
            let add_fn = rust_core::AddFn {};

            let environment = Rc::new(Environment::new_main_environment());
            environment.insert(Symbol::intern("+"), add_fn.to_rc_value());

            let plus: Rc<Value> = match &*environment {
                MainEnvironment(EnvironmentVal {
                    curr_ns_sym: _,
                    namespaces,
                }) => namespaces.get(&Symbol::intern("user"), &Symbol::intern("+")),
                _ => panic!("new_main_environment() should return Main"),
            };

            assert_eq!(
                8.to_value(),
                add_fn.invoke(vec![3_i32.to_rc_value(), 5_i32.to_rc_value()])
            );

            if let Value::IFn(add_ifn) = &*plus {
                assert_eq!(
                    8.to_value(),
                    add_ifn.invoke(vec![3_i32.to_rc_value(), 5_i32.to_rc_value()])
                );
                return;
            }
            panic!("plus should be IFn, is: {:#?}", plus);
        }
    }
}
