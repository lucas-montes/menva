use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_default_file() {
    read_env_file(".env");
}

pub fn read_env_file(filename: &str) {
    match File::open(filename) {
        Ok(file) => BufReader::new(file).lines().flatten().for_each(handle_line),
        Err(_) => panic!("Your {} has problems", filename),
    };
}

fn handle_line(line: String) {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
        match trimmed_line.split_once('=') {
            Some((key, value)) => env::set_var(key.trim(), value.trim()),
            None => panic!("Your line {} if malformated", trimmed_line),
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
