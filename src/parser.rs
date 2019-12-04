use std::fs;

pub fn parse_lines(args: Vec<String>) -> Vec<String> {
    parse_vals(args, read_lines)
}

pub fn parse_comma_list(args: Vec<String>) -> Vec<String> {
    parse_vals(args, read_comma_list)
}

fn parse_vals<F>(args: Vec<String>, parser: F) -> Vec<String>
where
    F: Fn(&str) -> std::io::Result<Vec<String>>,
{
    match args {
        _ if args.len() < 2 => {
            println!("Invalid arguments! (use '-f <filename>' or '<value> [, <value>]*'");
            std::process::exit(0);
        }
        _ if args[1] == "-f" && args.len() > 2 => parser(&args[2]).unwrap(),
        _ => args.into_iter().skip(1).collect(),
    }
}

fn read_comma_list(path: &str) -> std::io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .split(',')
        .map(str::trim)
        .filter_map(|s| match s.is_empty() {
            true => None,
            false => Some(String::from(s)),
        })
        .collect())
}

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect())
}
