use std::process::{exit, Command, Output};
use std::{fs, io};

pub fn link_to_obj(path: &String) -> String {
    let mut cmd = Command::new("llc");
    let obj_path = path.replace(".ll", ".o");

    cmd.arg("-filetype=obj");
    cmd.arg(path);
    cmd.arg("-o");
    cmd.arg(&obj_path);

    handle_output(cmd.output());
    println!("Linked to object file: '{}'", &obj_path);

    obj_path
}

pub fn link_to_binary(path: &String) {
    let mut cmd = Command::new("clang");
    let bin_path = path.replace(".ll", "");

    cmd.arg(path);
    cmd.arg("-o");
    cmd.arg(&bin_path);

    handle_output(cmd.output());
    println!("Linked to binary: '{}'", bin_path);
}

fn handle_output(output: io::Result<Output>) {
    match output {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}

pub fn remove_file(path: &String) {
    match fs::remove_file(path) {
        Ok(_) => println!("Removed file: '{}'", path),
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
