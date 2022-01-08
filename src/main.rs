use std::env;
use std::fs;
use std::process::Command;
//use configparser::ini::Ini;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()< 2 {
        println!("It needs the directory of the project passed as an argument");
    }
    else{
        let cmdline;
        let script;
        let cmd_arg;
        if cfg!(target_os = "windows"){
            script = "bat";
            cmdline = "cmd";
            cmd_arg = "/C";
        }
        else{
            cmdline = "sh";
            script = "sh";
            cmd_arg = "-c";
        }
        // Create a file inside a scope because i don't know how to
        // close it properly
        let filename = format!("run.{}",script);
        {
            let _result = fs::write(&filename,
                                    format!("cd \"{}\"\ncargo build", args[1]));
        }

        let argument = format!("{} \"cd \"{}\" & cargo build\"", cmd_arg, args[1]);
        println!("{}", argument);
        // Executing cargo build on directory
        let output =  Command::new(&cmdline)
                            .arg(argument)
                            //.arg(&cmd_arg)
                            //.arg(&filename)
                            .arg("cargo build")
                            .output()
                            .expect("failed to execute process");

        println!("Executing in cmd:");
        println!("{}", output.status);
        println!("{}", String::from_utf8_lossy(&output.stderr));

        // Read configuration
        //let mut config = Ini::new();
        //let map = config.load("./conf/app_settings.ini");
        //println!("{:?}", map);
    }
}
