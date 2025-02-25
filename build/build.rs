use std::env;
use std::fs::{self, File};
use std::io::Write;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let config_url = env::var("CONFIG_URL").expect("CONFIG_URL must be set");

    // Create the gen directory if it does not exist
    fs::create_dir_all("gen").unwrap();

    let mut file = File::create("gen/config.rs").unwrap();
    writeln!(file, "pub const CONFIG_URL: &str = \"{}\";", config_url).unwrap();
}
