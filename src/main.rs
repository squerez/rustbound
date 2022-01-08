use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Executing batch/sh selected file
    println!("Executing in cmd:");
    for arg in &args[1..]{
        println!("{}", arg);
    }
    let output = if cfg!(target_os = "windows"){
        Command::new("cmd")
                .args(&args[1..])
                .output()
                .expect("failed to execute process")
    }
    else{
        Command::new("sh")
                .args(&args[1..])
                .output()
                .expect("failed to execute process")
    };

    println!("{}", output.status);
}
