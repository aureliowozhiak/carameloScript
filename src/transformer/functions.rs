pub fn return_function_value(line: &str) -> String {
    let mut rust_code = String::new();
    let binding = line.replace("retorne", "");
    let return_value = binding.trim();
    rust_code.push_str(&format!("{}\n", return_value));
    rust_code
}

pub fn start_function(line: &str) -> String {
    let mut rust_code = String::new();
    let function_name = line.split_whitespace().nth(2).unwrap();
    rust_code.push_str(&format!("pub fn {}(", function_name));
    let parameters = line.split_whitespace().skip(4);
    let mut return_type = String::new();
    for parameter in parameters {
        let tmp_parameter = &format!("{}", parameter.replace(",", ""));
        //tmp_parameter -> exemplo: a:inteiro
        if tmp_parameter.contains(":") {
            let parts: Vec<&str> = tmp_parameter.split(":").collect();
            let parameter_name = parts[0];
            let parameter_type = parts[1];

            if parameter_name == "retorne" {
                return_type = (&parameter_type).to_string();
                continue;
            }
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
    
    if return_type != "" {
        rust_code.push_str(") -> ");
        if return_type == "inteiro" {
            rust_code.push_str("i32");
        } else if return_type == "real" {
            rust_code.push_str("f32");
        } else if return_type == "texto" {
            rust_code.push_str("&str");
        } else if return_type == "logico" {
            rust_code.push_str("bool");
        } else {
            panic!("Tipo de dado não suportado");
        }
        rust_code.push_str(" {\n");
    } else {
        rust_code.push_str(") {\n");
    }
    rust_code = rust_code.replace(" )", ")");
    rust_code
}