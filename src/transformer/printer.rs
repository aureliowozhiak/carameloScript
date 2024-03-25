pub fn print_fn(line: &str) -> String {
    let mut rust_code = String::new();
    
    let mut message = line.replace("escreva", "").trim().to_string();
            message = message.replace("(", "");
            message = message.replace(")", "");
            if message.starts_with("\"") {
                message = message.replace("\"", "");
                rust_code.push_str(&format!("println!(\"{}\");\n", message));
            } else {
                rust_code.push_str(&format!("println!(\"{}\", {});\n", "{}", message));
            }
    
    rust_code
}