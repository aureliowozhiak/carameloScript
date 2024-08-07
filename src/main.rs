mod transformer;

use std::{
    io::{Read, Write, BufReader},
    fs::{File, write},
    process::Output
};
use crate::transformer::transform;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Por favor, forneça o nome do arquivo a ser executado.");
        return;
    }

    let file_name: &String = &args[1];
    let file: File = File::open(file_name).unwrap();
    let mut content: String = String::new();
    BufReader::new(file).read_to_string(&mut content).unwrap();

    let _rust_code: String = transform(content);    
    write("tmp_script.rs", _rust_code).unwrap();

    let _output: Output = std::process::Command::new("rustc")
        .arg("tmp_script.rs")
        .output()
        .expect("Failed to compile the rust code");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut run_file: File = File::create("run.sh").unwrap();
        run_file.write_all(b"#!/bin/bash\n").unwrap();
        run_file.write_all(b"./tmp_script\n").unwrap();
        run_file.write_all(b"rm tmp_script.rs\n").unwrap();
        run_file.write_all(b"rm tmp_script\n").unwrap();
        run_file.write_all(b"rm run.sh\n").unwrap();
        std::fs::set_permissions("run.sh", std::fs::Permissions::from_mode(0o755)).unwrap();

        std::process::Command::new("sh")
            .arg("run.sh")
            .status()
            .expect("Failed to run the script");
    }

    #[cfg(windows)]
    {
        let mut run_file: File = File::create("run.bat").unwrap();
        run_file.write_all(b"@echo off\n").unwrap();
        run_file.write_all(b"tmp_script.exe\n").unwrap();
        run_file.write_all(b"del tmp_script.rs\n").unwrap();
        run_file.write_all(b"del tmp_script.exe\n").unwrap();
        run_file.write_all(b"del tmp_script.pdb\n").unwrap();
        run_file.write_all(b"del run.bat\n").unwrap();

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Start-Process cmd -ArgumentList '/c run.bat' -Verb RunAs")
            .status()
            .expect("Failed to execute elevated command");
    }
}
