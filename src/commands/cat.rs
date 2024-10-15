// src/commands/cat.rs

use std::fs::File;
use std::io::Read;

pub fn builtin_cat(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("cat: manque de fichier");
        return;
    }

    for filename in args {
        match File::open(filename) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    eprintln!("cat: {}: {}", filename, e);
                    continue;
                }
                print!("{}", contents);
            },
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
            },
        }
    }
}
