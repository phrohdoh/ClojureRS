use std::{fs, io};
use crate::{repl::ReadEval, value::Value};

pub fn try_eval_file(
    re: &dyn ReadEval,
    file_path: &str,
) -> io::Result<Option<Value>> {
    let f = fs::File::open(file_path)?;
    let mut rdr = io::BufReader::new(f);
    Ok(re.eval_exhaustive(&mut rdr))
}
