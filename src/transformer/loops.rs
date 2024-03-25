pub fn start_while(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("enquanto", "").trim().replace("faca", "");
    let condition = binding.trim();
    rust_code.push_str(&format!("while {} {{\n", condition));
    rust_code
}