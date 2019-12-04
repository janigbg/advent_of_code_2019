use std::fs;

pub fn parse_lines(args: Vec<String>) -> Vec<String> {
    match args {
        _ if args.len() < 2 => {
            println!("Invalid arguments! (use '-f <filename>' or '<value> [, <value>]*'");
            std::process::exit(0);
        },
        _ if args[1] == "-f" && args.len() > 2 => read_lines(&args[2]).unwrap(),
        _ => args.into_iter().skip(1).collect(),
    }
}

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect())
}