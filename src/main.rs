use std::io::Read;

fn transform(content: String) -> String {
    let mut rust_code = String::new();
    rust_code.push_str("fn main() {\n");

    // the file contains this:
    // escreva("Teste")
    //soma = 1 + 1
    //escreva(soma)

    for line in content.lines() {
        if line.starts_with("escreva") {
            let mut message = line.replace("escreva", "").trim().to_string();
            message = message.replace("(", "");
            message = message.replace(")", "");

            if message.starts_with("\"") {
                message = message.replace("\"", "");
                rust_code.push_str(&format!("println!(\"{}\");\n", message));
            }else{
                rust_code.push_str(&format!("println!(\"{}\", {});\n", "{}", message));
            }
            
            
        }else if line.contains("=") {
            let mut parts = line.split("=");
            let variable = parts.next().unwrap().trim();
            let expression = parts.next().unwrap().trim();
            rust_code.push_str(&format!("let mut {} = {};\n", variable, expression));
        }
        
    }

    rust_code.push_str("}");
    rust_code
}

fn main() {

    // read the parameters from the command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide the name of the file to be executed");
        return;
    }

    let file_name = &args[1];

    let file = std::fs::File::open(file_name).unwrap();

    let mut content = String::new();
    std::io::BufReader::new(file).read_to_string(&mut content).unwrap();
    
    let _rust_code = transform(content);
    
    std::fs::write("tmp_script.rs", _rust_code).unwrap();

    // compile the rust code
    let _output = std::process::Command::new("rustc")
        .arg("tmp_script.rs")
        .output()
        .expect("Failed to compile the rust code");

    // execute the compiled code
    let _output = std::process::Command::new("./tmp_script")
        .output()
        .expect("Failed to execute the compiled code");

    // print the output
    println!("{}", String::from_utf8_lossy(&_output.stdout));

    // remove the temporary files
    std::fs::remove_file("tmp_script.rs").unwrap();
    std::fs::remove_file("tmp_script").unwrap();
        
}