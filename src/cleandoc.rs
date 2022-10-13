use textwrap;

pub fn cleandoc(docstring: &str) -> String {
    // split the docstring into lines
    let lines = docstring.lines().collect::<Vec<&str>>();
    // Find minimum indentation of any non-blank lines after first line.
    let mut margin = u64::MAX;

    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();

        margin = std::cmp::min(margin, indent as u64);
    }

    // Remove left whitespace from first line.
    let first_line = lines[0].trim_start().to_string();

    let rest = lines
        .iter()
        .skip(1)
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let dedented = textwrap::dedent(&rest);

    (first_line + "\n" + &dedented).trim().to_string()
}

// tests

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works_with_basic_docstring() {
        let a_docstring = r#"
        Example docstring
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring() {
        let a_docstring = r#"
            Example docstring
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_leading_newline() {
        let a_docstring = r#"

            Example docstring
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_trailing_newline() {
        let a_docstring = r#"
            Example docstring

        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines() {
        let a_docstring = r#"
            Example docstring
            with multiple lines
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\nwith multiple lines");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines_separated_by_multiple_lines() {
        let a_docstring = r#"
            Example docstring


            with multiple lines
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\n\n\nwith multiple lines");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines_different_indentation() {
        let a_docstring = r#"
            Example docstring
                with multiple lines
        "#;
        let result = super::cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\n    with multiple lines");
    }
}
