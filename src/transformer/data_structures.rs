
// my list in file is: lista numeros = [1, 2, 3, 4, 5]
pub fn list(line: &str) -> String {
    let mut rust_code = String::new();
    let mut parts = line.split("=");
    let variable = parts.next().unwrap().trim();
    let expression = parts.next().unwrap().trim();
    let expression = expression.replace("[", "");
    let expression = expression.replace("]", "");
    let expression = expression.replace(" ", "");
    let expression = expression.replace(",", ", ");
    rust_code.push_str(&format!("let mut {} = vec![{}];\n", 
                        &variable.replace("lista ", ""), 
                        expression));
    rust_code
}