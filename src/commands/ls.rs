use std::fs;
use std::path::Path;

pub fn builtin_ls(args: Vec<&str>) {
    let mut show_all = false;
    let mut long_format = false;
    let mut classify = false;
    let mut path = Path::new(".");

    // Parsing des arguments
    for arg in &args {
        if arg.starts_with("-") {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => eprintln!("ls: option inconnue -- '{}'", c),
                }
            }
        } else {
            path = Path::new(arg);
        }
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    if !show_all && file_name_str.starts_with('.') {
                        continue;
                    }

                    if long_format {
                        let metadata = match entry.metadata() {
                            Ok(metadata) => metadata,
                            Err(e) => {
                                eprintln!("ls: {}: {}", file_name_str, e);
                                continue;
                            },
                        };
                        let file_type = if metadata.is_dir() { "d" } else { "-" };
                        println!("{} {}", file_type, file_name_str);
                    } else if classify {
                        let metadata = match entry.metadata() {
                            Ok(metadata) => metadata,
                            Err(e) => {
                                eprintln!("ls: {}: {}", file_name_str, e);
                                continue;
                            },
                        };
                        let suffix = if metadata.is_dir() { "/" } else { "" };
                        println!("{}{}", file_name_str, suffix);
                    } else {
                        println!("{}", file_name_str);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("ls: {}: {}", path.display(), e);
        },
    }
}
