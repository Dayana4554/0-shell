use std::env;

pub fn builtin_cd(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("cd: manque de répertoire");
        return;
    }

    if let Err(e) = env::set_current_dir(args[0]) {
        eprintln!("cd: {}: {}", args[0], e);
    }
}
