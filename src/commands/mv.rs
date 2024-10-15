// src/commands/mv.rs

use std::fs;
use std::path::Path;

pub fn builtin_mv(args: Vec<&str>) {
    if args.len() < 2 {
        eprintln!("mv: syntaxe incorrecte. Utilisation: mv source destination");
        return;
    }

    let source = args[0];
    let destination = args[1];

    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    match fs::rename(source_path, destination_path) {
        Ok(_) => (),
        Err(e) => eprintln!("mv: {}: {}", source, e),
    }
}
