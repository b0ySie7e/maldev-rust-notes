use utils::hello_from_utils;

fn main() {
    println!("Implant: {}", hello_from_utils());
    println!("2 + 3 = {}", utils::add(2, 3));
}