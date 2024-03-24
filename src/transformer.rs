fn print_fn(line: &str) -> String {
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

fn expression_fn(line: &str) -> String {
    let mut rust_code = String::new();

    let mut parts = line.split("=");
    let variable = parts.next().unwrap().trim();
    let expression = parts.next().unwrap().trim();
    rust_code.push_str(&format!("let {} = {};\n", variable, expression));

    rust_code
}

//inicio funcao parametros: a, b
//escreva(a + b)
//escreva("fim")
//fim funcao

fn start_function_fn(line: &str) -> String {
    let mut rust_code = String::new();
    let function_name = line.split_whitespace().nth(2).unwrap();
    rust_code.push_str(&format!("fn {}(", function_name));
    
    let parameters = line.split_whitespace().skip(4);
    for parameter in parameters {
        let tmp_parameter = &format!("{}", parameter.replace(",", ""));
        //tmp_parameter -> exemplo: a:inteiro
        if tmp_parameter.contains(":") {
            let parts: Vec<&str> = tmp_parameter.split(":").collect();
            let parameter_name = parts[0];
            let parameter_type = parts[1];
            rust_code.push_str(parameter_name);
            if parameter_type == "inteiro" {
                rust_code.push_str(": i32");
            } else if parameter_type == "real" {
                rust_code.push_str(": f32");
            } else if parameter_type == "texto" {
                rust_code.push_str(": &str");
            } else if parameter_type == "logico" {
                rust_code.push_str(": bool");
            } else {
                panic!("Tipo de dado não suportado");
            }
        }
        rust_code.push_str(", ")
    }

    // remove the last comma
    rust_code = rust_code.trim_end_matches(',').to_string();
    
    rust_code.push_str(") {\n");
    rust_code = rust_code.replace(" )", ")");
    rust_code
}

fn end_function_fn() -> String {
    let mut rust_code = String::new();
    rust_code.push_str("}\n");
    rust_code
}
    

pub fn transform(content: String) -> String {
    let mut rust_code = String::new();
    for line in content.lines() {
        let line = line.trim();
        if line == "inicio codigo principal" {
            rust_code.push_str("// código principal\n");
            rust_code.push_str("fn main() {\n");
            continue;
        }
        if line == "inicio funcoes" {
            continue;
        }
        match line {
            line if line.starts_with("escreva") => {
                rust_code.push_str(&print_fn(line));
            },
            line if line.contains("=") => {
                rust_code.push_str(&expression_fn(line));
            },
            line if line.starts_with("inicio funcao") => {
                rust_code.push_str(&start_function_fn(line));
            },
            line if line.starts_with("fim funcao") => {
                rust_code.push_str(&end_function_fn());
            },
            // se não der match e se a linha não estiver vazia apenas adiciona a linha ao código
            line if !line.is_empty() => {
                rust_code.push_str(&format!("{};\n", line));
            },
            _ => {}
        }
    }
    rust_code.push_str("}");
    rust_code
}
