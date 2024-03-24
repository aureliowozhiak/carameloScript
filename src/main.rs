use std::io::Read;
// from transform.rs file import the struct Transformer impl Transformer 
mod transformer;

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
    
    let _rust_code = transformer::transform(content);
    
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