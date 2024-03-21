use redis::{ShellRunner, ShellRunnerContainer};
use std::io::{self, Write};

mod actions;

fn main() {
    let mut runner: ShellRunnerContainer = ShellRunner::new();

    // This loop handles user input
    loop {
        print!("❯ ");
        let _ = io::stdout().flush(); // Flush screen to show >

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        runner.run(input.trim());
    }
}
