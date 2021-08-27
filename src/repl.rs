use std::io::BufRead;
use std::sync::Arc;

use crate::environment::Environment;
use crate::reader;
use crate::runtime;
use crate::value::{Evaluable, Value};

pub trait ReadEval {
    fn read_single(&self, reader: &mut dyn BufRead) -> Option<Value>;
    fn eval(&self, val: &Value) -> Value;
    fn eval_exhaustive(&self, reader: &mut dyn BufRead) -> Option<Value>;
}
impl ReadEval for Repl {
    fn read_single(&self, reader: &mut dyn BufRead) -> Option<Value> {
        Repl::read(&mut Box::new(reader))
    }
    fn eval(&self, val: &Value) -> Value {
        Repl::eval(self, val)
    }
    fn eval_exhaustive(&self, reader: &mut dyn BufRead) -> Option<Value> {
        let mut most_recently_evaled_val = None;
        //
        while let Some(val) = ReadEval::read_single(self, reader) {
            most_recently_evaled_val = match val {
                Value::Condition(_) => break,
                _ => ReadEval::eval(self, &val),
            }.into();
        }
        //
        most_recently_evaled_val
    }
}

/*
pub trait ReadEval {
    fn read_single(&self, reader: &mut dyn BufRead) -> Option<Value>;
    fn eval_exhaustive(&self, reader: &mut dyn BufRead) -> Option<Value> {
        <Self as ReadEval>::read_single(self, reader)
            .map(|val| repl.eval(&val))
    }
}
pub trait ReadEvalFile: ReadEval {
    fn eval_file(
        &self,
        repl: &Repl,
        file_path: AsRef<std::path::Path>,
    ) -> Option<Value> {
        let f = std::fs::File::open(file_path.as_ref())?;
        let rdr = std::io::BufReader::new(f);
        <Self as ReadEval>::eval(
            self,
            &mut rdr,
            repl,
        )
    }
}
impl ReadEval for Repl {
    fn read(&self, reader: &mut dyn BufRead) -> Option<Value> {
        reader::read(&mut Box::new(reader))
    }
}
*/

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
    pub fn eval_file(&self, filepath: &str) -> Option<Value> {
        match runtime::try_eval_file(self, filepath) {
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
            Some(Value::PersistentList(_)) => {}
            _ => panic!("Reading of integer should have returned Value::PersistentList"),
        }

        let vector = Repl::read_string("[1 2 a]");
        match vector {
            Some(Value::PersistentVector(_)) => {}
            _ => panic!("Reading of integer should have returned Value::PersistentVector"),
        }

        let symbol = Repl::read_string("abc");
        match symbol {
            Some(Value::Symbol(_)) => {}
            _ => panic!("Reading of integer should have returned Value::Symbol"),
        }
    }
}
