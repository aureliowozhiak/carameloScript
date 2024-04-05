pub fn start_if(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("se", "").trim().replace("entao", "");
    let condition = binding.trim();
    rust_code.push_str(&format!("if {} {{\n", condition));
    rust_code
}

pub fn start_else_if(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("senao se", "").trim().replace("entao", "");
    let condition = binding.trim();
    rust_code.push_str("} ");
    rust_code.push_str(&format!("else if {} {{\n", condition));
    rust_code
}

pub fn start_match(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("caso", "");
    let condition = binding.trim();
    rust_code.push_str(&format!("match {} {{\n", condition));
    rust_code
}

pub fn start_match_condition(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("seja", "").trim().replace("entao", "");
    let condition = binding.trim();
    rust_code.push_str(&format!("{} => {{\n", condition));
    rust_code
}