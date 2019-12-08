use std::fs;

pub fn print_args(args: &Vec<String>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

pub fn parse_lines(args: &Vec<String>) -> Vec<String> {
    parse_vals(args, read_lines)
}

pub fn parse_comma_list(args: &Vec<String>) -> Vec<String> {
    parse_vals(args, read_comma_list)
}

pub fn parse_digits(args: &Vec<String>) -> Vec<String> {
    parse_vals(args, read_digits)
}

fn parse_vals<F>(args: &Vec<String>, parser: F) -> Vec<String>
where
    F: Fn(String) -> Vec<String>,
{
    match args {
        _ if args.len() < 2 => {
            println!("Invalid arguments! (use '-f <filename>' or '<value> [, <value>]*'");
            std::process::exit(0);
        }
        _ if args[1] == "-f" && args.len() > 2 => parser(read_from_file(&args[2]).unwrap()),
        _ => args.clone().into_iter().skip(1).collect(),
    }
}

pub fn read_digits(s: String) -> Vec<String> {
    s.chars()
        .map(|c| String::from(format!("{}", c)))
        .collect()
}

pub fn read_comma_list(s: String) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter_map(|s| match s.is_empty() {
            true => None,
            false => Some(String::from(s)),
        })
        .collect()
}

pub fn read_lines(s: String) -> Vec<String> {
    s.lines().map(String::from).collect()
}

fn read_from_file(path: &str) -> std::io::Result<String> {
    Ok(fs::read_to_string(path)?)
}
