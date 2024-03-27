fn main() {
let mut nomes = vec!["João", "Maria", "José", "Pedro", "Paulo", "Lucas", "Mateus", "Marcos", "Luciana", "Mariana", "Joaquim", "Joaquina", "Joaquino", "Joaquinete", "Joaquimzinho"];
let mut quantidade_nomes = 15;
let mut x = 0;
while x < quantidade_nomes {
println!("{}", nomes[x]);
x = x + 1;
}
println!("Fim do programa");
let mut nome = String::new();;
println!("Digite seu nome: ");;
std::io::stdin().read_line(&mut nome).unwrap();;
println!("Seu nome é: {}", nome);;
}
