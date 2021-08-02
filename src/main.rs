use std::io;
use rust_clojure::{
    user_action::{Action, parse_args},
    repl::Repl,
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
        Action::RunScript(script_filepath) => {
            if let Some(val) = repl.eval_file(script_filepath.as_str()) {
                println!("{}", val);
            }
        }

        // eval the expression
        Action::Evaluate(expr_str) => {
            if let Some(last_val) = repl.eval_readable(expr_str.as_bytes()) {
                println!("{}", last_val);
            }
        }

        // Start repl
        Action::Nothing => {
           let input = io::stdin();
           let output = io::stdout();
           repl.run(input.lock(), output.lock());
        }
    }
}
