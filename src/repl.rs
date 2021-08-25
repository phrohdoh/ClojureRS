use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

use crate::environment::Environment;
use crate::reader;
use crate::value::{Evaluable, Value};

pub struct Repl {
    pub environment: Arc<Environment>,
}
impl Repl {
    pub fn new(environment: Arc<Environment>) -> Repl {
        Repl { environment }
    }

    // Just wraps reader's read
    pub fn read<R: BufRead>(reader: &mut R) -> Option<Value> {
        reader::read(reader)
    }
    // @TODO add to reader.rs and wrap here
    pub fn read_string(string: &str) -> Option<Value> {
        Repl::read(&mut string.as_bytes())
    }

    // @TODO reconsider eval's signature;  since Value wraps all evaluables,  it might make more sense
    // to frame eval as "environment.eval(value)", and then likewise define a
    // 'repl.eval(value)', rather than 'value.eval(environment)'
    pub fn eval(&self, value: &Value) -> Value {
        value.eval(Arc::clone(&self.environment))
    }
    //
    // Will possibly just add this to our environment, or turn this into a parallel of clojure.lang.RT
    //
    /// Reads the code in a file sequentially and evaluates the result
    pub fn try_eval_file(&self, filepath: &str) -> Result<Option<Value>, std::io::Error> {
        let core = File::open(filepath)?;
        let reader = BufReader::new(core);
        Ok(self.eval_readable(reader))
    }
    pub fn eval_file(&self, filepath: &str) -> Option<Value> {
        match self.try_eval_file(filepath) {
            Ok(Some(v)) => Some(v),
            Ok(None) => None,
            Err(e) => Some(Value::Condition(e.to_string())),
        }
    }
    /// Reads code sequentially and evaluates the result, returning the last value
    pub fn eval_readable<R: BufRead>(&self, mut r: R) -> Option<Value> {
        let mut last_val = Repl::read(&mut r);
        loop {
            match last_val {
                None => return None,
                Some(last_val) => {
                    // @TODO this is hardcoded until we refactor Conditions to have keys, so that
                    //       we can properly identify them
                    // @FIXME
                    if let Value::Condition(cond) = &last_val {
                        if cond == "Tried to read empty stream; unexpected EOF" {
                            return None;
                        }

                        println!("Error reading string: {}", cond);
                        return Some(last_val);
                    }

                    let evaled_last_val = self.eval(&last_val); let line = line!();

                    if let Value::Condition(cond) = evaled_last_val {
                        println!("[{}:{}] {}", file!(), line, cond);
                    }
                }
            }

            last_val = Repl::read(&mut r);
        }
    }

    pub fn run<I, O>(
        &self,
        mut input: I,
        mut output: O,
    ) where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        loop {
            let _ = write!(&mut output, "{}=> ", self.environment.get_current_namespace_name());
            let _ = output.flush();

            let next = {
                // Read
                let next = Repl::read(&mut input);

                if_chain::if_chain! {
                    if let Some(Value::Keyword(kw)) = &next;
                    if let Some("repl") = kw.namespace();
                    if kw.name() == "quit";
                    then { return; }
                }

                next
            };

            if let Some(next) = next {
                // Eval
                let evaled_next = self.eval(&next);
                // Print
                println!("{}", evaled_next);
            }
            // Loop
        }
    }
}

impl Default for Repl {
    fn default() -> Repl {
        Repl {
            environment: Environment::clojure_core_environment(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repl::Repl;
    use crate::value::Value;
    //@TODO separate into individual tests
    #[test]
    fn read_string() {
        let num = Repl::read_string("1");
        match num {
            Some(Value::I32(_)) => {}
            _ => panic!("Reading of integer should have returned Value::I32"),
        }
        let list = Repl::read_string("(+ 1 2)");
        match list {
            Some(Value::List(_)) => {}
            _ => panic!("Reading of list should have returned Value::List"),
        }

        let vector = Repl::read_string("[1 2 a]");
        match vector {
            Some(Value::Vector(_)) => {}
            _ => panic!("Reading of vector should have returned Value::Vector"),
        }

        let symbol = Repl::read_string("abc");
        match symbol {
            Some(Value::Symbol(_)) => {}
            _ => panic!("Reading of symbol should have returned Value::Symbol"),
        }
    }
}
