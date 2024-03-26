pub fn read_string(line: &str) -> String {
    let mut rust_code = String::new();
    let variable = line.replace("leia", "");
    let binding = variable.trim();
    rust_code.push_str(&format!("let mut {} = String::new();\n", binding));
    rust_code.push_str(&format!("std::io::stdin().read_line(&mut {}).unwrap();\n", binding));
    rust_code.push_str(&format!("{} = {}.trim().parse().unwrap();\n", binding, binding));
    rust_code
}