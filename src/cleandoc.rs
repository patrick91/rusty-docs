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
    // Remove margin from other lines.
    let mut other_lines = lines[1..]
        .iter()
        .map(|line| {
            if line.len() < margin as usize {
                line.to_string()
            } else {
                line[margin as usize..].to_string()
            }
        })
        .collect::<Vec<String>>();

    // Reassemble the docstring.
    // join first line with other lines using vectors
    let mut result = vec![first_line];
    result.append(&mut other_lines);

    // Remove empty lines from the end.
    while result.last().map_or(false, |line| line.trim().is_empty()) {
        result.pop();
    }

    // Remove empty lines from the beginning.
    while result.first().map_or(false, |line| line.trim().is_empty()) {
        result.remove(0);
    }
    result.join("\n")
}

// tests

#[cfg(test)]
mod tests {
    use crate::cleandoc::cleandoc;

    #[test]
    fn it_works_with_basic_docstring() {
        let a_docstring = r#"
        Example docstring
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring() {
        let a_docstring = r#"
            Example docstring
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_leading_newline() {
        let a_docstring = r#"

            Example docstring
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_trailing_newline() {
        let a_docstring = r#"
            Example docstring

        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines() {
        let a_docstring = r#"
            Example docstring
            with multiple lines
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\nwith multiple lines");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines_separated_by_multiple_lines() {
        let a_docstring = r#"
            Example docstring


            with multiple lines
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\n\n\nwith multiple lines");
    }

    #[test]
    fn it_works_with_indented_docstring_with_multiple_lines_different_indentation() {
        let a_docstring = r#"
            Example docstring
                with multiple lines
        "#;
        let result = cleandoc(a_docstring);
        assert_eq!(result, "Example docstring\n    with multiple lines");
    }
}
