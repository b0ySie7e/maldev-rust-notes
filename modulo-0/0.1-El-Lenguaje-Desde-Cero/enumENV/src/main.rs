use std::env;
fn main() {
    for (key, values) in env::vars() {
        println!("{}: {}", key, values);
    }
}
