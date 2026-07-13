use utils::{hello_from_utils, get_hostname};

fn main() {
    println!("{}", hello_from_utils());
    println!("Hostname: {}", get_hostname());
}