use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <PID>", args[0]);
        return;
    }

    let input = &args[1];

    let pid = match input.parse::<u32>() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Ingresá un PID válido (número)");
            return;
        }
    };
    
    let proc_path = format!("/proc/{}", pid);

    if !Path::new(&proc_path).exists() {
        eprintln!("No existe proceso con PID {}", pid);
        return;
    }

    let comm = fs::read_to_string(format!("/proc/{}/comm", pid))
        .unwrap_or("desconocido".to_string());
    
    let cmdline = fs::read_to_string(format!("/proc/{}/cmdline", pid))
    .unwrap_or("desconocido".to_string());
    
    let status = fs::read_to_string(format!("/proc/{}/status", pid))
        .unwrap_or("desconocido".to_string());

    println!("PID:     {}", pid);
    println!("Nombre:  {}", comm.trim());
    println!("Cmdline: {}", cmdline.trim());
    println!("{}", status);

}