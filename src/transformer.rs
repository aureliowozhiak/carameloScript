
mod conditionals;
mod functions;
mod loops;
mod printer;
mod utils;
mod reader;
mod data_structures;
mod parser;

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
            line if line.starts_with("senao se ") => {
                rust_code.push_str(&conditionals::start_else_if(line));
            },
            line if line.starts_with("senao") => {
                rust_code.push_str("} else {\n");
            },line if line.starts_with("fim se") =>{
                rust_code.push_str(&utils::end_with_keys());
            }line if line.starts_with("fim funcao") => {
                rust_code.push_str(&utils::end_with_keys());
            },line if line.starts_with("fim enquanto") => {
                rust_code.push_str(&utils::end_with_keys());
            }, line if line.starts_with("fim para") => {
                rust_code.push_str(&utils::end_with_keys());
            },
            line if line.starts_with("para") => {
                rust_code.push_str(&loops::start_for(line));
            },
            line if line.starts_with("enquanto") => {
                rust_code.push_str(&loops::start_while(line));
            },
            line if line.starts_with("incremente") => {
                rust_code.push_str(&utils::increment(line));
            },
            line if line.starts_with("decremente") => {
                rust_code.push_str(&utils::decrement(line));
            },
            line if line.starts_with("leia") => {
                rust_code.push_str(&reader::read_string(line));
            },
            line if line.starts_with("lista") => {
                rust_code.push_str(&data_structures::list(line));
            },
            line if line.contains("=") => {
                rust_code.push_str(&utils::expression(line));
            },
            // converte para inteiro a_string b_inteiro
            line if line.contains("converta para inteiro") => {
                rust_code.push_str(&parser::string_to_int(line));
            },
            line if line.contains("converta para real") => {
                rust_code.push_str(&parser::string_to_float(line));
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