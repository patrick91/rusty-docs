use crate::extract;
use std::{fs, path::{Path, PathBuf, Component}};

use minijinja::{context, Environment};

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

pub fn generate(code: &str) -> String {
    let module = extract::extract(code);

    let path = Path::new("./templates/function.md");

    let path = if path.exists() {
        path
    } else {
        Path::new("./node_modules/rusty_docs/templates/function.md")
    };

    println!("normalised path is: {:?}", normalize_path(path));

    let function_template = fs::read_to_string(path).expect("Unable to read function template");

    let mut env = Environment::new();
    env.add_template("function", &function_template).unwrap();

    let function_template = env.get_template("function").unwrap();

    let mut output = String::new();

    for function in module.functions {
        let content = function_template
            .render(context!(
                function_name => function.name,
                function_docstring => function.docstring,
                function_arguments => function.arguments
            ))
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
