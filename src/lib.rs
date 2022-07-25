use std::env;
use std::fs::File;
use std::io::{self, Read};
pub struct UserArgs {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl UserArgs {
    pub fn from(query: String, filename: String) -> Self {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        UserArgs {
            query: query,
            filename: filename,
            ignore_case: ignore_case,
        }
    }
}
pub fn get_input_args(args: &mut Vec<String>, iteration: i32) -> Result<UserArgs, io::ErrorKind> {
    if args.len() > 1 && iteration == 1 {
        Ok(UserArgs::from(args[1].to_owned(), args[2].to_owned()))
    } else {
        retry_input()
    }
}

pub fn retry_input() -> Result<UserArgs, io::ErrorKind> {
    eprintln!("Type again: [query] [filename]");

    let mut s = String::new();

    match io::stdin().read_line(&mut s) {
        Ok(_) => eprintln!("reading line success"),
        Err(err) => panic!("reading line failure {}", err),
    };

    let items: Vec<String> = s
        .split(' ')
        .map(|s| s.to_owned().trim().to_string())
        .collect();

    if items.len() != 2 || !s.contains(' ') {
        eprintln!("expected 2 arguments");
        return Err(io::ErrorKind::InvalidInput);
    }

    Ok(UserArgs::from(items[0].to_owned(), items[1].to_owned()))
}

pub fn open_file_and_read<'a>(
    ignore_case: bool,
    write_file: &'a mut String,
    filename: &str,
    query: &str,
) -> Result<Vec<&'a str>, io::Error> {
    let mut f = File::open(filename)?;

    f.read_to_string(write_file)?;

    let result = if ignore_case {
        search_insensitive(query, write_file)
    } else {
        search(query, write_file)
    };

    if result.len() > 0 {
        Ok(result)
    } else {
        panic!("No matching query found")
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    fn search_case_insensitive() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_insensitive(query, contents)
        );
    }
}
