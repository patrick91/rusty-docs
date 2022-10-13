use crate::cleandoc;
use textwrap;

struct Docstring {
    pub title: String,
    pub description: String,
    // TODO: make structs for these
    pub arguments: String,
    pub returns: String,
    pub raises: String,
}

impl Docstring {
    pub fn new_from_string(docstring: &str) -> Self {
        let cleaned_docstring = cleandoc::cleandoc(docstring);

        let mut description = String::new();
        let mut arguments = String::new();
        let mut returns = String::new();
        let mut raises = String::new();

        let mut current_section = &mut description;

        let mut lines = cleaned_docstring.lines();

        // the title starts with the first line and ends when the first empty line is encountered
        // pop lines until the first empty line is encountered
        let title = lines
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .collect::<Vec<&str>>()
            .join(" ");

        // let title_lines = lines.take_while(|line| !line.trim().is_empty());
        // let title = title_lines.collect::<Vec<&str>>().join("\n");

        for line in lines {
            if line.starts_with("Args:") {
                current_section = &mut arguments;
                continue;
            } else if line.starts_with("Returns:") {
                current_section = &mut returns;
                continue;
            } else if line.starts_with("Raises:") {
                current_section = &mut raises;
                continue;
            }

            current_section.push_str(line);
            current_section.push('\n')
        }

        Self {
            title,
            description: description.trim().to_string(),
            arguments: textwrap::dedent(&arguments).trim().to_string(),
            returns: textwrap::dedent(&returns).trim().to_string(),
            raises: textwrap::dedent(&raises).trim().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};

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

        assert_eq!(
            parsed_docstring.arguments,
            "table_handle: An open smalltable.Table instance.\nkeys: A sequence of strings representing the key of each table\n  row to fetch.  String keys will be UTF-8 encoded.\nrequire_all_keys: If True only rows with values set for all keys will be\n  returned."
        );

        assert_eq!(
            parsed_docstring.returns,
            "A dict mapping keys to the corresponding table row data\nfetched. Each row is represented as a tuple of strings. For\nexample:\n\n{b'Serak': ('Rigel VII', 'Preparer'),\n b'Zim': ('Irk', 'Invader'),\n b'Lrrr': ('Omicron Persei 8', 'Emperor')}\n\nReturned keys are always bytes.  If a key from the keys argument is\nmissing from the dictionary, then that row was not found in the\ntable (and require_all_keys must have been False)."
        );

        assert_eq!(
            parsed_docstring.raises,
            "IOError: An error occurred accessing the smalltable."
        );
    }
}
