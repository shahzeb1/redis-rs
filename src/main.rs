use rebis::{create_data_container, run};
use std::io::{self, Write};

mod actions;

fn main() {
    let mut data = create_data_container();

    // This loop handles user input
    loop {
        print!("❯ ");
        let _ = io::stdout().flush(); // Flush screen to show >

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        run(input.trim(), &mut data);
    }
}
