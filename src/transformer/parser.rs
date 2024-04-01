pub fn string_to_int(line: &str) -> String {
    let mut rust_code = String::new();
    // converte para inteiro string_value int_value
    let variable = line.replace("converte para inteiro", "");
    let parts: Vec<&str> = variable.split(" ").collect();
    let string_value = parts[1];
    let int_value = parts[2];
    rust_code.push_str(&format!("let {}: u32 = {}.parse().unwrap();\n", int_value, string_value));
    rust_code
}

pub fn string_to_float(line: &str) -> String {
    let mut rust_code = String::new();
    // converte para inteiro string_value int_value
    let variable = line.replace("converte para real", "");
    let parts: Vec<&str> = variable.split(" ").collect();
    let string_value = parts[1];
    let float_value = parts[2];
    rust_code.push_str(&format!("let {}: f32 = {}.parse().unwrap();\n", float_value, string_value));
    rust_code
}