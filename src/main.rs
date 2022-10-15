use std::fs;

mod cleandoc;
mod docstrings;
mod extract;
mod generate;

fn main() {
    let code =
        fs::read_to_string("./src/fixtures/strawberry_type.py").expect("Unable to read file");

    println!("{}", generate::generate(&code));
}
