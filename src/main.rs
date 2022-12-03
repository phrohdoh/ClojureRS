use rust_clojure::{
    user_action,
    repl,
};

fn main() {
    let cli_args: user_action::Action = user_action::parse_args(std::env::args().collect());

    // instantiate the core environment
    let repl = repl::Repl::default();

    match cli_args {
        // eval the file/script
        user_action::Action::RunScript(script) => {
            println!("{}", repl::Repl::eval_file(&repl, script.as_str()));
        }

        // eval the expression
        user_action::Action::Evaluate(expression) => {
            println!(
                "{}",
                repl::Repl::eval(&repl, &repl::Repl::read_string(&expression))
            );
        }

        // Start repl
        user_action::Action::Nothing => {
            repl.run();
        }
    }
}
