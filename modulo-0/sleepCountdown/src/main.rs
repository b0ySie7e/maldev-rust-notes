// 4. **Sleep loop con countdown visible** — Implementa delay con feedback al usuario. Fundamento de técnicas de evasión por tiempo.
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    println!("[+] Iniciando el countdown de 10 segundos");
    for i in 1..=10 {
        print!("\n {} segundos restantes...", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n[+] completado")
}
