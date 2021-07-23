use rust_clojure::{
    user_action::{Action, parse_args},
    repl::{self, Repl},
};

fn main() {
    let cli_arg: Action = parse_args(std::env::args().collect());

    // instantiate the core environment
    let repl = Repl::default();

    // do the work
    act(&repl, cli_arg);
}

fn act(repl: &Repl, a: Action) {
    match a {
        // eval the file/script
        Action::RunScript(_script) => unimplemented!("init script not supported; usee '-e'"),
        // todo: wasi
        //Action::RunScript(script) => {
        //    println!("{}", Repl::eval_file(&repl, script.as_str()));
        //}

        // eval the expression
        Action::Evaluate(expr_str) => {
            // let expr_val = Repl::read_string(&expr_str);
            // let evaled = Repl::eval(&repl, &expr_val);
            let last_val = repl.eval_readable(expr_str.as_bytes());
            println!("{}", last_val);
        }

        // Start repl
        Action::Nothing => unimplemented!("repl not supported; use '-e'"),
        // todo: wasi (stdio)
        // Action::Nothing => {
        //    repl.run();
        // }
    }
}
