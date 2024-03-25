pub fn expression_fn(line: &str) -> String {
    let mut rust_code = String::new();

    let mut parts = line.split("=");
    let variable = parts.next().unwrap().trim();
    let expression = parts.next().unwrap().trim();
    rust_code.push_str(&format!("let mut {} = {};\n", variable, expression));

    rust_code
}

pub fn end_with_keys() -> String {
    let mut rust_code = String::new();
    rust_code.push_str("}\n");
    rust_code
}