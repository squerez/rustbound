use configparser::ini::Ini;

fn main() {
    let number = 123;
    println!("Hello, world! {}", number);
    
    // Read configuration
    let mut config = Ini::new();
    let map = config.load("../conf/app_settings.ini");
    println!("{:?}", map);
}
