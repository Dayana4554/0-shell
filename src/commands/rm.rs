// src/commands/rm.rs

use std::fs;
use std::path::Path;

pub fn builtin_rm(args: Vec<&str>) {
    let mut recursive = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        eprintln!("rm: aucun fichier spécifié");
        return;
    }

    for path in paths {
        let path = Path::new(path);
        if path.is_dir() {
            if recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => (),
                    Err(e) => eprintln!("rm: {}: {}", path.display(), e),
                }
            } else {
                eprintln!("rm: {}: est un répertoire", path.display());
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => (),
                Err(e) => eprintln!("rm: {}: {}", path.display(), e),
            }
        }
    }
}
