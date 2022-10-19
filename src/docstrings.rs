use crate::cleandoc;
use serde::Serialize;
use textwrap;

#[derive(Debug, Serialize)]
pub struct Argument {
    pub name: String,
    pub type_: Option<String>,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Raises {
    pub exception: String,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum BodyPart {
    Text(String),
    CodeSnippet(String),
}

#[derive(Serialize, Debug)]
pub struct Docstring {
    pub title: String,
    pub description: String,
    pub returns: String,
    pub body: Vec<BodyPart>,
    pub arguments: Vec<Argument>,
    pub private_arguments: Vec<Argument>,
    pub raises: Vec<Raises>,
}

fn parse_arguments(docstring: &str) -> Vec<Argument> {
    let ds = textwrap::dedent(docstring);

    let mut arguments = Vec::new();
    let mut lines = ds.lines();

    let mut current_argument: Option<Argument> = None;

    // for each line in the docstring, if it has a colon, it's an argument
    // otherwise, it's part of the argument's description

    while let Some(line) = lines.next() {
        if line.contains(':') {
            // if we have a current argument, push it to the list
            if let Some(argument) = current_argument {
                arguments.push(argument);
            }

            // start a new argument
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim().to_string();

            let description = parts.next().map(|s| s.trim().to_string());
            current_argument = Some(Argument {
                name,
                type_: None,
                default: None,
                description,
            });
        } else {
            // this should not happen, but if it does, just ignore it
            if current_argument.is_none() {
                continue;
            }

            // if we have a current argument, add this line to its description
            let current_argument = current_argument.as_mut().unwrap();
            let description = current_argument
                .description
                .take()
                .unwrap_or_else(|| "".to_string());
            current_argument.description = Some((description + "\n" + line).trim().to_owned());
        }
    }

    // push the last argument
    if let Some(argument) = current_argument {
        arguments.push(argument);
    }

    arguments
}

fn parse_raises(docstring: &str) -> Vec<Raises> {
    let ds = textwrap::dedent(docstring);

    let mut raises = Vec::new();
    let mut lines = ds.lines();

    let mut current_raises: Option<Raises> = None;

    // for each line in the docstring, if it has a colon, it's an argument
    // otherwise, it's part of the argument's description

    while let Some(line) = lines.next() {
        if line.contains(':') {
            // if we have a current argument, push it to the list
            if let Some(r) = current_raises {
                raises.push(r);
            }

            // start a new argument
            let mut parts = line.splitn(2, ':');
            let exception = parts.next().unwrap().trim().to_string();

            let description = parts.next().map(|s| s.trim().to_string());
            current_raises = Some(Raises {
                exception,
                description,
            });
        } else {
            // this should not happen, but if it does, just ignore it
            if current_raises.is_none() {
                continue;
            }

            // if we have a current argument, add this line to its description
            let current_raises = current_raises.as_mut().unwrap();
            let description = current_raises
                .description
                .take()
                .unwrap_or_else(|| "".to_string());
            current_raises.description = Some((description + "\n" + line).trim().to_owned());
        }
    }

    // push the last argument
    if let Some(r) = current_raises {
        raises.push(r);
    }

    raises
}

impl Docstring {
    pub fn new_from_string(docstring: &str) -> Self {
        let cleaned_docstring = cleandoc::cleandoc(docstring);

        let mut description = String::new();
        let mut arguments = String::new();
        let mut private_arguments = String::new();
        let mut returns = String::new();
        let mut raises = String::new();

        let mut current_section = &mut description;

        let mut current_section_type = "body";

        let mut lines = cleaned_docstring.lines();

        // the title starts with the first line and ends when the first empty line is encountered
        // pop lines until the first empty line is encountered
        let title = lines
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .collect::<Vec<&str>>()
            .join(" ");

        let mut body: Vec<BodyPart> = Vec::new();
        let mut current_body_part: Option<BodyPart> = None;

        for line in lines {
            if current_section_type == "body" {
                if line.starts_with(">>> ") {
                    // if we have a part and it's a CodeSnippet we can append to it, otherwise we need to create a new one

                    let code = &line[4..];

                    match current_body_part {
                        Some(BodyPart::CodeSnippet(ref mut snippet)) => {
                            snippet.push('\n');
                            snippet.push_str(code);
                        }
                        Some(BodyPart::Text(part)) => {
                            if !part.trim().is_empty() {
                                body.push(BodyPart::Text(part));
                            }

                            current_body_part = Some(BodyPart::CodeSnippet(code.to_string()));
                        }
                        None => {
                            current_body_part = Some(BodyPart::CodeSnippet(code.to_string()));
                        }
                    }
                } else {
                    match current_body_part {
                        Some(BodyPart::Text(ref mut text)) => {
                            text.push('\n');
                            text.push_str(line);
                        }
                        Some(part) => {
                            body.push(part);
                            current_body_part = Some(BodyPart::Text(line.to_string()));
                        }
                        None => {
                            current_body_part = Some(BodyPart::Text(line.to_string()));
                        }
                    }
                }
            }

            if line.starts_with("Args:") || line.starts_with("Arguments:") {
                current_section_type = "arguments";
                current_section = &mut arguments;
                continue;
            } else if line.starts_with("Private arguments:") {
                current_section_type = "private_arguments";
                current_section = &mut private_arguments;
                continue;
            } else if line.starts_with("Returns:") {
                current_section_type = "returns";
                current_section = &mut returns;
                continue;
            } else if line.starts_with("Raises:") {
                current_section_type = "raises";
                current_section = &mut raises;
                continue;
            }

            current_section.push_str(line);
            current_section.push('\n')
        }

        if let Some(part) = current_body_part {
            body.push(part);
        }

        let arguments = parse_arguments(&arguments);
        let raises = parse_raises(&raises);
        let private_arguments = parse_arguments(&private_arguments);

        Self {
            title,
            description: description.trim().to_string(),
            arguments,
            private_arguments,
            body,
            returns: textwrap::dedent(&returns).trim().to_string(),
            raises,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::docstrings::BodyPart;

    #[test]
    fn it_parses_docstrings() {
        let docstring = r#"
        Fetches rows from a Smalltable.

        Retrieves rows pertaining to the given keys from the Table instance
        represented by table_handle.  String keys will be UTF-8 encoded.

        Args:
            table_handle: An open smalltable.Table instance.
            keys: A sequence of strings representing the key of each table
              row to fetch.  String keys will be UTF-8 encoded.
            require_all_keys: If True only rows with values set for all keys will be
              returned.

        Returns:
            A dict mapping keys to the corresponding table row data
            fetched. Each row is represented as a tuple of strings. For
            example:

            {b'Serak': ('Rigel VII', 'Preparer'),
             b'Zim': ('Irk', 'Invader'),
             b'Lrrr': ('Omicron Persei 8', 'Emperor')}

            Returned keys are always bytes.  If a key from the keys argument is
            missing from the dictionary, then that row was not found in the
            table (and require_all_keys must have been False).

        Raises:
            IOError: An error occurred accessing the smalltable.
        "#;

        let parsed_docstring = super::Docstring::new_from_string(docstring);

        assert_eq!(parsed_docstring.title, "Fetches rows from a Smalltable.");
        assert_eq!(
            parsed_docstring.description,
            "Retrieves rows pertaining to the given keys from the Table instance\nrepresented by table_handle.  String keys will be UTF-8 encoded."
        );

        assert_eq!(parsed_docstring.arguments.len(), 3);
        assert_eq!(parsed_docstring.arguments[0].name, "table_handle");
        assert_eq!(
            parsed_docstring.arguments[0].description,
            Some("An open smalltable.Table instance.".to_string())
        );
        assert_eq!(parsed_docstring.arguments[0].default, None);
        assert_eq!(parsed_docstring.arguments[0].type_, None);

        assert_eq!(parsed_docstring.arguments[1].name, "keys");
        assert_eq!(parsed_docstring.arguments[1].description, Some("A sequence of strings representing the key of each table\n  row to fetch.  String keys will be UTF-8 encoded.".to_string()));
        assert_eq!(parsed_docstring.arguments[1].default, None);
        assert_eq!(parsed_docstring.arguments[1].type_, None);

        assert_eq!(parsed_docstring.arguments[2].name, "require_all_keys");
        assert_eq!(
            parsed_docstring.arguments[2].description,
            Some("If True only rows with values set for all keys will be\n  returned.".to_string())
        );
        assert_eq!(parsed_docstring.arguments[2].default, None);
        assert_eq!(parsed_docstring.arguments[2].type_, None);

        assert_eq!(
            parsed_docstring.returns,
            "A dict mapping keys to the corresponding table row data\nfetched. Each row is represented as a tuple of strings. For\nexample:\n\n{b'Serak': ('Rigel VII', 'Preparer'),\n b'Zim': ('Irk', 'Invader'),\n b'Lrrr': ('Omicron Persei 8', 'Emperor')}\n\nReturned keys are always bytes.  If a key from the keys argument is\nmissing from the dictionary, then that row was not found in the\ntable (and require_all_keys must have been False)."
        );

        assert_eq!(parsed_docstring.raises.len(), 1);

        assert_eq!(parsed_docstring.raises[0].exception, "IOError");
        assert_eq!(
            parsed_docstring.raises[0].description,
            Some("An error occurred accessing the smalltable.".to_string())
        );
    }

    #[test]
    fn it_parses_docstrings_with_code_snippets() {
        let docstring = r#"
        This is a docstring with code snippets

        >>> 1 + 1 = 2
        >>> 2 + 2 = 4
        >>> print("something")

        >>> 1 + 1 = 3
        >>> 2 + 2 = 5
        >>> print("something wrong")
        "#;

        let parsed_docstring = super::Docstring::new_from_string(docstring);

        assert_eq!(
            parsed_docstring.title,
            "This is a docstring with code snippets"
        );

        assert_eq!(parsed_docstring.body.len(), 2);

        assert_eq!(
            parsed_docstring.body[0],
            BodyPart::CodeSnippet("1 + 1 = 2\n2 + 2 = 4\nprint(\"something\")".to_string())
        );

        assert_eq!(
            parsed_docstring.body[1],
            BodyPart::CodeSnippet("1 + 1 = 3\n2 + 2 = 5\nprint(\"something wrong\")".to_string())
        );
    }
}
