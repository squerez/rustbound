use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    executable = args[1];
    println!("The arguments are:");
    /*for arg in args{
        println!("{}", arg);
    }*/

}
