pub fn builtin_echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}