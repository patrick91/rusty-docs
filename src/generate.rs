use crate::extract;
use std::fs;

use minijinja::{context, Environment};

pub fn generate(code: &str) -> String {
    let module = extract::extract(code);

    let function_template = fs::read_to_string("./src/templates/function.md")
        .expect("Unable to read function template");

    let mut env = Environment::new();
    env.add_template("function", &function_template).unwrap();

    let function_template = env.get_template("function").unwrap();

    let mut output = String::new();

    for function in module.functions {
        let content = function_template
            .render(context!(function_name => function.name, function_docstring => function.docstring))
            .unwrap();

        output.push_str(&content);
    }

    output
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_file_with_only_functions() {
        let code = fs::read_to_string("./src/fixtures/functions.py").expect("Unable to read file");

        insta::assert_debug_snapshot!(super::generate(&code));
    }
}
