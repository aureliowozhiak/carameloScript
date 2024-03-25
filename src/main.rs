use std::io::Read;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
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
        
    // create run.sh file
    let mut run_file = std::fs::File::create("run.sh").unwrap();
    run_file.write_all(b"#!/bin/bash\n").unwrap();
    run_file.write_all(b"./tmp_script\n").unwrap();
    run_file.write_all(b"rm tmp_script.rs\n").unwrap();
    run_file.write_all(b"rm tmp_script\n").unwrap();
    run_file.write_all(b"rm run.sh\n").unwrap();
    std::fs::set_permissions("run.sh", std::fs::Permissions::from_mode(0o755)).unwrap();

        
}