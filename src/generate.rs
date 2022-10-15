use crate::extract;

pub fn generate(code: &str) -> String {
    let module = extract::extract(code);

    // TODO: jinja templates

    let mut output = String::new();

    for function in module.functions {
        output.push('#');
        output.push(' ');
        output.push_str(&function.name);
        output.push('\n');
        output.push('\n');
        output.push('#');
        output.push('#');
        output.push(' ');
        output.push_str(&function.docstring.title);
        output.push('\n');
        output.push('\n');

        let description = &function.docstring.description;

        if description != "" {
            output.push_str(description);
            output.push('\n');
            output.push('\n');
        }

        // TODO: make this a map and maybe merge with type hints ?
        let arguments = &function.docstring.arguments;

        if !arguments.is_empty() {
            output.push_str("## Arguments:");
            output.push('\n');
            output.push('\n');

            for argument in arguments {
                output.push_str("- **");
                output.push_str(&argument.name);
                output.push_str(":** ");

                // TODO: call this description
                match &argument.documentation {
                    Some(argument_documentation) => {
                        output.push_str(&argument_documentation);
                    }
                    None => {}
                }

                output.push('\n');
            }

            output.push('\n');
            output.push('\n');
        }

        if !function.docstring.returns.is_empty() {
            output.push_str("## Returns:");
            output.push('\n');
            output.push('\n');

            output.push_str(&function.docstring.returns);
        }
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
