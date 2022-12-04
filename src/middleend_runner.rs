use std::process::Command;

pub fn run_opt(path: &String) {
    let mut cmd = Command::new("opt");

    cmd.arg(path);
    cmd.arg("-S");
    cmd.arg("-o");
    cmd.arg(path);

    cmd.output().unwrap();
    println!("Optimized file: '{}'", path);
}
