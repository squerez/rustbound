use std::fs;
use std::env;
use std::path::Path;
use std::fs;
use std::process::Command;
use configparser::ini::Ini;

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

        // Remove temporary files and create temporary folder
        let tmp_path = "./.tmp/";
        let path_exists: bool = Path::new(tmp_path).is_dir();

        if path_exists {
            fs::remove_dir_all(tmp_path).unwrap();
        }
        fs::create_dir(tmp_path).unwrap();

        // Create a file inside a scope because i don't know how to
        // close it properly
        let filename = format!("{}/run.{}", tmp_path, script);
        {
            let _result = fs::write(&filename,
                                    format!("cd \"{}\"\ncargo build", args[1]));
        }

        // Executing cargo build on directory
        let output =  Command::new(&cmdline)
                            .arg(&cmd_arg)
                            .arg(&filename)
                            .output()
                            .expect("failed to execute process");

        println!("Executing in cmd:");
        println!("{}", String::from_utf8_lossy(&output.stderr));

        // Read configuration
        let mut config = Ini::new();
        let map = config.load("./conf/app_settings.ini");
        println!("{:?}", map);
    }
}
