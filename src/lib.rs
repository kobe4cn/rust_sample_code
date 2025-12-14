use rand::Rng;
use std::{cmp::Ordering, env};
#[allow(unused)]
fn guess() {
    let mut rng = rand::rng();
    let secert_number: u32 = rng.random_range(1..=100);
    loop {
        println!("Your guess number: (type 'quit' to exit)");
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "quit" {
            println!("You quit the game!");
            break;
        }
        // parse the guess number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // compare the guess number with the secert number
        match guess.cmp(&secert_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

use anyhow::Result;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(anyhow::anyhow!("Didn't get a query string")),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(anyhow::anyhow!("Didn't get a file path")),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore_case: {}", ignore_case);
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
use std::fs;
pub fn run(config: Config) -> Result<()> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents: {}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn one_result_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}
