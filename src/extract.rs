use crate::docstrings;
use rustpython_ast::{ArgData, Arguments, Constant, ExprKind, Located, StmtKind};
use rustpython_parser::parser;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Argument {
    pub name: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    // TODO: arguments?
    pub docstring: docstrings::Docstring,
    pub arguments: Vec<Argument>,
}

pub struct Module {
    // pub name: String,
    // pub docstring: docstrings::Docstring,
    pub functions: Vec<Function>,
}

fn extract_function(
    name: String,
    body: Vec<Located<StmtKind>>,
    arguments: Box<Arguments>,
) -> Function {
    // find docstring, the first statement in the body of the function
    // that's an Expr with a Constant value

    let docstring_text = match body.first() {
        Some(stmt) => match &stmt.node {
            StmtKind::Expr { value } => match &value.node {
                ExprKind::Constant { value, kind: _ } => match value {
                    Constant::Str(value) => value.clone(),
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            },
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let docstring = docstrings::Docstring::new_from_string(&docstring_text);

    let docstring_arguments = docstring
        .arguments
        .iter()
        .map(|arg| (arg.name.clone(), arg))
        .collect::<std::collections::HashMap<String, &docstrings::Argument>>();

    let mut function_arguments = Vec::new();

    // args, posonlyargs and kwonlyargs are lists of arg nodes.
    // vararg and kwarg are single arg nodes, referring to the *args, **kwargs parameters.
    // kw_defaults is a list of default values for keyword-only arguments. If one is None, the corresponding argument is required.
    // defaults is a list of default values for arguments that can be passed positionally. If there are fewer defaults, they correspond to the last n arguments.

    for argument in arguments.args {
        match argument.node {
            ArgData {
                arg,
                annotation: Some(annotation),
                ..
            } => {
                let name = arg.to_string();
                let description = match docstring_arguments.get(&name) {
                    Some(arg) => arg.description.clone(),
                    None => None,
                };

                let argument = Argument {
                    name,
                    type_: annotation.to_string(),
                    default: None,
                    description,
                };
                function_arguments.push(argument);
            }
            _ => {}
        }
    }
    let kw_missing_defaults = arguments.kwonlyargs.len() - arguments.kw_defaults.len();

    for (index, argument) in arguments.kwonlyargs.iter().enumerate() {
        let default = if index >= kw_missing_defaults {
            Some(&arguments.kw_defaults[index - kw_missing_defaults])
        } else {
            None
        };

        match &argument.node {
            ArgData {
                arg,
                annotation: Some(annotation),
                ..
            } => {
                let name = arg.to_string();
                let description = match docstring_arguments.get(&name) {
                    Some(arg) => arg.description.clone(),
                    None => None,
                };

                let argument = Argument {
                    name,
                    type_: annotation.to_string(),
                    default: match default {
                        Some(default) => Some(default.to_string()),
                        None => None,
                    },
                    description,
                };
                function_arguments.push(argument);
            }
            _ => {}
        }
    }

    Function {
        name: name.to_string(),
        docstring,
        arguments: function_arguments,
    }
}

pub fn extract(code: &str) -> Module {
    let python_ast = parser::parse_program(&code, "something").expect("Unable to parse");

    let mut functions = Vec::new();

    // find all functions in ast
    for statement in python_ast {
        match statement.node {
            StmtKind::FunctionDef {
                name,
                body,
                args,
                decorator_list: _,
                returns: _,
                type_comment: _,
            } => {
                functions.push(extract_function(name, body, args));
            }
            _ => {}
        }
    }

    Module { functions }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_parses_python_function() {
        let code = r#"
        def foo(a: int, b: int) -> int:
            """Example docstring

            Args:
                a: a number
                b: another number

            Returns:
                the sum of a and b

            Raises:
                ValueError: if a or b are not numbers
            """
            return a + b
        "#;

        let result = super::extract(&textwrap::dedent(code));

        assert_eq!(result.functions.len(), 1);

        let function = &result.functions[0];

        assert_eq!(function.name, "foo");
        assert_eq!(function.docstring.title, "Example docstring");
        assert_eq!(function.docstring.description, "");
        assert_eq!(function.docstring.arguments.len(), 2);

        let arguments = &function.docstring.arguments;

        assert_eq!(arguments[0].name, "a");
        assert_eq!(arguments[0].description, Some("a number".to_string()));
        assert_eq!(arguments[1].name, "b");
        assert_eq!(arguments[1].description, Some("another number".to_string()));

        assert_eq!(function.docstring.returns, "the sum of a and b");
        assert_eq!(function.docstring.raises.len(), 1);
        assert_eq!(function.docstring.raises[0].exception, "ValueError");
        assert_eq!(
            function.docstring.raises[0].description,
            Some("if a or b are not numbers".to_string())
        );
    }
}
