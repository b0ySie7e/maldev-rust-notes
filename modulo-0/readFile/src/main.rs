use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";

    let file = match File::open(file_path){
        Ok(f)=> f,
        Err(e) => {
            eprint!("Error al abrir el archivo: {}", e);
            return;
        }
    };
    
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("{}: {}", i + 1, content),
            Err(e) => eprintln!("Error al leer línea {}: {}", i + 1, e),
        }
    }

}
