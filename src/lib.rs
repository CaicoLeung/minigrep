use clap::Parser;
use colored::Colorize;
use core::str;
use std::{error::Error, fs, path::PathBuf};

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

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,
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
