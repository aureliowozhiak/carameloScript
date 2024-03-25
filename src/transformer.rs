
mod conditionals;
mod functions;
mod loops;
mod printer;
mod utils;

pub fn transform(content: String) -> String {
    let mut rust_code = String::new();
    for line in content.lines() {
        let line = line.trim();
        if line == "inicio codigo principal" {
            rust_code.push_str("fn main() {\n");
            continue;
        }
        if line == "fim codigo principal" {
            rust_code.push_str(&utils::end_with_keys());
            continue;
        }
        if line == "inicio funcoes" || line == "fim funcoes" {
            continue;
        }
        match line {
            line if line.starts_with("escreva") => {
                rust_code.push_str(&printer::print_fn(line));
            },
            line if line.starts_with("inicio funcao") => {
                rust_code.push_str(&functions::start_function(line));
            },
            line if line.starts_with("retorne") => {
                rust_code.push_str(&functions::return_function_value(line));
            },
            line if line.starts_with("se ") => {
                rust_code.push_str(&conditionals::start_if(line));
            },
            line if line.starts_with("senao") => {
                rust_code.push_str("} else {\n");
            },line if line.starts_with("fim se") =>{
                rust_code.push_str(&utils::end_with_keys());
            }line if line.starts_with("fim funcao") => {
                rust_code.push_str(&utils::end_with_keys());
            },line if line.starts_with("fim enquanto") => {
                rust_code.push_str(&utils::end_with_keys());
            },
            line if line.starts_with("enquanto") => {
                rust_code.push_str(&loops::start_while(line));
            },
            line if line.starts_with("incremente") => {
                let variable = line.replace("incremente", "");
                let binding = variable.trim();
                rust_code.push_str(&format!("{} = {} + 1;\n", binding, binding));
            },
            line if line.starts_with("decremente") => {
                let variable = line.replace("decremente", "");
                let binding = variable.trim();
                rust_code.push_str(&format!("{} = {} - 1;\n", binding, binding));
            },
            line if line.starts_with("leia") => {
                let variable = line.replace("leia", "");
                let binding = variable.trim();
                rust_code.push_str(&format!("let mut {} = String::new();\n", binding));
                rust_code.push_str(&format!("std::io::stdin().read_line(&mut {}).unwrap();\n", binding));
                rust_code.push_str(&format!("{} = {}.trim().parse().unwrap();\n", binding, binding));
            },
            line if line.contains("=") => {
                rust_code.push_str(&utils::expression_fn(line));
            },
            line if !line.is_empty() => {
                rust_code.push_str(&format!("{};\n", line));
            },
            line if line.starts_with("#") => {
                continue;
                //rust_code.push_str(&format!("// {}\n", line));
            },
            _ => {}
        }
    }
    rust_code
}
