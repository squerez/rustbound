use std::env;
use std::process::Command;
use configparser::ini::Ini;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()< 2 {
        println!("It needs the directory of the project passed as an argument");
    } else {
        let cmdstr = format!("cd {} && cargo build", args[1]);
        
        let cmdline;
        let argument;
        
        // Different configs for windows or linux
        if cfg!(target_os = "windows"){
            cmdline = "cmd";
            argument = ["/K", &cmdstr] 
        } else {
            cmdline = "/usr/bin/sh";
            argument = ["-c", &cmdstr] 
       }

        // Executing cargo build on directory
        let output =  Command::new(cmdline)
                                .args(&argument)
                                .output()
                                .expect("failed to execute process");

        println!("{}", String::from_utf8_lossy(&output.stderr));

        // Read configuration
        let mut config = Ini::new();
        let map = config.load("./conf/app_settings.ini");
        println!("{:?}", map);
    }
}
