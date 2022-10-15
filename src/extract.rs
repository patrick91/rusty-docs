use crate::docstrings;
use rustpython_ast::{Constant, ExprKind, StmtKind};
use rustpython_parser::parser;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    // TODO: arguments?
    pub docstring: docstrings::Docstring,
}

pub struct Module {
    // pub name: String,
    // pub docstring: docstrings::Docstring,
    pub functions: Vec<Function>,
}

pub fn extract(code: &str) -> Module {
    let python_ast = parser::parse_program(&code, "something").expect("Unable to parse");

    let mut functions = Vec::new();

    // find all functions in ast
    for statement in python_ast {
        match statement.node {
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => {
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

                functions.push(Function {
                    name: name.to_string(),
                    docstring,
                });
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

        let function = result.functions[0].clone();

        assert_eq!(function.name, "foo");
        assert_eq!(function.docstring.title, "Example docstring");
        assert_eq!(function.docstring.description, "");
        assert_eq!(function.docstring.arguments.len(), 2);

        let arguments = function.docstring.arguments.clone();

        assert_eq!(arguments[0].name, "a");
        assert_eq!(arguments[0].description, Some("a number".to_string()));
        assert_eq!(arguments[1].name, "b");
        assert_eq!(
            arguments[1].description,
            Some("another number".to_string())
        );

        assert_eq!(function.docstring.returns, "the sum of a and b");
        assert_eq!(function.docstring.raises.len(), 1);
        assert_eq!(function.docstring.raises[0].exception, "ValueError");
        assert_eq!(
            function.docstring.raises[0].description,
            Some("if a or b are not numbers".to_string())
        );
    }
}
