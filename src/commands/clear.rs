// src/commands/clear.rs
use std::io::Write;

pub fn builtin_clear(_args: Vec<&str>) {
    // Envoyer la séquence d'échappement ANSI pour effacer l'écran et repositionner le curseur
    print!("\x1B[2J\x1B[H");
    // Assurez-vous que la séquence est envoyée immédiatement
    std::io::stdout().flush().unwrap();
}