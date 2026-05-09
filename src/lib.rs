use colored::Colorize;
use core::str;
use std::{env, error::Error, fs, usize};

#[cfg(test)]
mod tests {
    use super::*;

    const CONTEXTS: &str = "\
Rust:
Safe, Fast, Productive.
Pick three.
";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["Safe, Fast, Productive."], search(query, &CONTEXTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, &CONTEXTS));
    }
}

fn format_line((line, content): (usize, &str), query: &str, ignore_case: bool) -> String {
    let colored = if ignore_case {
        // Case-insensitive: find and replace the actual matched substring
        if let Some(pos) = content.to_lowercase().find(&query.to_lowercase()) {
            let actual = &content[pos..pos + query.len()];
            content.replacen(actual, &actual.red().to_string(), 1)
        } else {
            content.to_string()
        }
    } else {
        content.replacen(query, &query.red().to_string(), 1)
    };
    format!("{}: {}", line.to_string().green(), colored)
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<String> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| content.to_lowercase().contains(&query.to_lowercase()))
        .map(|(i, content)| format_line((i + 1, content), query, true))
        .collect()
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<String> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| content.contains(query))
        .map(|(i, content)| format_line((i + 1, content), query, false))
        .collect::<Vec<_>>()
}

pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'a str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args.get(1).expect("Missing query argument");
        let file_path = &args.get(2).expect("Missing file_path argument");
        let ignore_case = matches!(
            &env::var("IGNORE_CASE").map(|v| v.to_lowercase()).as_deref(),
            Ok("yes" | "true" | "y")
        );
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(
    Config {
        query,
        file_path,
        ignore_case,
    }: Config,
) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path).expect("Should have benen to read the file");
    let result = match ignore_case {
        true => search_case_insensitive(&query, &content),
        false => search(&query, &content),
    };
    let result_colorized = result.join("\n");
    println!("{}", result_colorized);
    Ok(result.join("\n"))
}
