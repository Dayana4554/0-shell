// src/commands/cp.rs

use std::fs;
use std::path::Path;

pub fn builtin_cp(args: Vec<&str>) {
    if args.len() < 2 {
        eprintln!("cp: syntaxe incorrecte. Utilisation: cp source destination");
        return;
    }

    let source = args[0];
    let destination = args[1];

    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if source_path.is_dir() {
        eprintln!("cp: {}: est un répertoire", source);
        return;
    }

    match fs::copy(source_path, destination_path) {
        Ok(_) => (),
        Err(e) => eprintln!("cp: {}: {}", source, e),
    }
}
