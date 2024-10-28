use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub use menva_macros::FromEnv;

pub fn read_default_file() {
    read_env_file(".env")
}

pub fn read_env_file(filename: impl AsRef<Path> + std::fmt::Debug) {
    match File::open(&filename) {
        Ok(file) => BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .for_each(handle_line),
        Err(err) => panic!("Your {filename:?} has problems: {err}"),
    };
}

fn handle_line(line: String) {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
        match trimmed_line.split_once('=') {
            Some((key, value)) => env::set_var(key.trim(), value.trim()),
            None => panic!("Your line {trimmed_line} if malformated"),
        }
    };
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or(String::from(""))
}

pub fn get_bool_env(key: &str) -> bool {
    get_env(key).parse::<bool>().unwrap_or(false)
}

pub fn get_int_env(key: &str) -> i64 {
    get_env(key).parse::<i64>().unwrap_or(0)
}
