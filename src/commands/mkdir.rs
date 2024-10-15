// src/commands/mkdir.rs

use std::fs;

pub fn builtin_mkdir(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("mkdir: aucun répertoire spécifié");
        return;
    }

    for dir in args {
        match fs::create_dir(dir) {
            Ok(_) => (),
            Err(e) => eprintln!("mkdir: {}: {}", dir, e),
        }
    }
}
