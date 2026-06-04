use rust_template_project::Greeter;
use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    let greeter = Greeter::new(name);
    println!("{}", greeter.greet());
    println!("{}", greeter.greet_formal());
}
