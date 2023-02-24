use crate::extract;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

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

// multline string
const TEMPLATE: &str = r#"
# {{ function_name }}

{{ function_docstring.title }}
{% for part in function_docstring.body -%}
{%-if part.CodeSnippet -%}
```python
{{ part.CodeSnippet }}
```
{%-elif part.Text %}
{{ part.Text }}
{% endif %}
{% endfor-%}

{% if function_docstring.arguments %}
## Arguments:


| Name | Type | Description |
| --- | --- | --- |
{% for argument in function_arguments-%}
| {{ argument.name }} | {{ argument.type }} | {{ argument.description }} |
{% endfor %}
{% endif %}

{% if function_docstring.returns %}
## Returns:

{{ function_docstring.returns }}
{% endif %}

{% if function_docstring.raises %}
## Raises:

{{ function_docstring.raises }}
{% endif %}
"#;

pub fn generate(code: &str) -> String {
    let module = extract::extract(code);

    // let path = Path::new("./templates/function.md");

    // let path = if path.exists() {
    //     path
    // } else {
    //     Path::new("./node_modules/rusty_docs/templates/function.md")
    // };

    // println!("normalised path is: {:?}", normalize_path(path));

    let function_template = TEMPLATE; // fs::read_to_string(path).expect("Unable to read function template");

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

// #[cfg(test)]
// mod tests {

//     #[cfg(test)]
//     glob!("./src/fixtures/*.py", |path| {
//         let code = fs::read_to_string(path).unwrap();

//         insta::assert_display_snapshot!(super::generate(&code));
//     });
// }

#[test]
fn test_snapshots() {
    insta::glob!("./src/fixtures/**/**/.py", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_display_snapshot!(&contents);
    });
}
// use insta::glob;

// #[cfg(test)]
// glob!("./src/fixtures/*.py", |path| {
//     let code = fs::read_to_string(path).unwrap();

//     insta::assert_display_snapshot!(super::generate(&code));
// });
