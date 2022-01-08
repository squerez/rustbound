use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use configparser::ini::Ini;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()< 2 {
        println!("It needs the directory of the project passed as an argument");
    }
    else{
        let cmdline;
        if cfg!(target_os = "windows"){
            cmdline = "cmd";
        }
        else{
            cmdline = "sh";
        }

        // Remove temporary files and create temporary folder
        let tmp_path = "./.tmp/";
        let path_exists: bool = Path::new(tmp_path).is_dir();

        if path_exists {
            fs::remove_dir_all(tmp_path).unwrap();
        }
        fs::create_dir(tmp_path).unwrap();

        // Executing cargo build on directory
        let output =  Command::new(cmdline)
                            .arg(format!("cd {}", args[1]))
                            .arg("cargo build")
                            .output()
                            .expect("failed to execute process");

        println!("Executing in cmd:");
        println!("{}", output.status);
        println!("{}", String::from_utf8_lossy(&output.stdout));

        // Read configuration
        let mut config = Ini::new();
        let map = config.load("./conf/app_settings.ini");
        println!("{:?}", map);
    }
}
