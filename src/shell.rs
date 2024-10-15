// src/shell.rs

use std::io::{self, Write};
use crate::commands; // Importer le module commands

pub fn run_shell() {
    loop {
        // Affichage du prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Lecture de la commande
        let mut input = String::new();
        let bytes_read = match io::stdin().read_line(&mut input) {
            Ok(n) => n,
            Err(error) => {
                eprintln!("Erreur de lecture: {}", error);
                continue;
            },
        };

        // Gestion de Ctrl+D (EOF)
        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Parsing de la commande
        let (command, args) = parse_command(input);

        // Exécution de la commande
        match command {
            "cd" => commands::cd::builtin_cd(args),
            "exit" => commands::exit_cmd::builtin_exit(),
            "echo" => commands::echo::builtin_echo(args),
            "pwd" => commands::pwd::builtin_pwd(),
            "ls" => commands::ls::builtin_ls(args),
            "cat" => commands::cat::builtin_cat(args),
            "cp" => commands::cp::builtin_cp(args),
            "rm" => commands::rm::builtin_rm(args),
            "mv" => commands::mv::builtin_mv(args),
            "mkdir" => commands::mkdir::builtin_mkdir(args),
            "clear" => commands::clear::builtin_clear(args),
            _ => {
                eprintln!("Commande '{}' non trouvée", command);
            },
        }
    }
}

fn parse_command(input: &str) -> (&str, Vec<&str>) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let command = tokens.get(0).unwrap_or(&"");
    let args = tokens[1..].to_vec();
    (*command, args)
}
