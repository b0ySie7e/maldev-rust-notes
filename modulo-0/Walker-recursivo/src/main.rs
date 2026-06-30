// **Walker de directorio recursivo** — Recorre `C:\Users\` listando todos los `.txt`, `.docx`, `.pdf` encontrados. Base de recolección de datos.

use std::fs;
use std::path::PathBuf;

fn main() {
    let start_path = "C:\\Users";
    let extensions = vec!["txt", "docx", "pdf"];

    match scan_directory(start_path, &extensions) {
        Ok(count) => println!("\nTotal archivos encontrados: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn scan_directory(path: &str, extensions: &[&str]) -> Result<usize, String> {
    let mut count = 0;

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let path = e.path();

                        
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                let ext_str = ext.to_string_lossy().to_lowercase();
                                if extensions.contains(&ext_str.as_str()) {
                                    println!("{}", path.display());
                                    count += 1;
                                }
                            }
                        }

                        if path.is_dir() {
                            match scan_directory(path.to_str().unwrap_or(""), extensions) {
                                Ok(c) => count += c,
                                Err(_) => {} 
                            }
                        }
                    }
                    Err(_) => {} 
                }
            }
        }
        Err(e) => return Err(format!("No se pudo acceder a {}: {}", path, e)),
    }

    Ok(count)
}
