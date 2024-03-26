pub fn expression(line: &str) -> String {
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

pub fn increment(line: &str) -> String {
    let mut rust_code = String::new();
    let variable = line.replace("incremente", "");
    let binding = variable.trim();
    rust_code.push_str(&format!("{} = {} + 1;\n", binding, binding));
    rust_code
}

pub fn decrement(line: &str) -> String {
    let mut rust_code = String::new();
    let variable = line.replace("decremente", "");
    let binding = variable.trim();
    rust_code.push_str(&format!("{} = {} - 1;\n", binding, binding));
    rust_code
}