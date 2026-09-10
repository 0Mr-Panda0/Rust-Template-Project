use divan::black_box;
use rust_template_project::Greeter;

fn main() {
    divan::main();
}

#[divan::bench]
fn new_greeter() {
    let _ = Greeter::new(black_box("Ferris"));
}

#[divan::bench]
fn greet_casual() {
    let greeter = Greeter::new(black_box("Ferris")).unwrap();
    let _ = black_box(greeter.greet());
}

#[divan::bench]
fn greet_formal() {
    let greeter = Greeter::new(black_box("Ferris")).unwrap();
    let _ = black_box(greeter.greet_formal());
}
