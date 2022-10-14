pub fn generate(code: &str) -> String {
    // let module = extract::extract(code);

    "implement me".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_file_with_only_functions() {
        // open file fixtures/functions.py as a string

        let code = fs::read_to_string("./src/fixtures/functions.py").expect("Unable to read file");

        insta::assert_debug_snapshot!(super::generate(&code));
    }
}
