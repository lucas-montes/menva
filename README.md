# Menva

Set your environment variables from your .env file
```rust
use menva::read_env_file;

fn main() {
    read_env_file(".env");
    do_cool_things();
}
```


Get your environment variables with their type
```rust
use menva::{get_env, get_bool_env};

pub fn do_cool_things() {
    match get_bool_env("SHOULD_I") {
        true => println!("{}", get_env("SECRET_TEXT")),
        false => todo!(),
    }
}
```

You can also use it with a struct
```rust
use menva::FromEnv;

#[derive(Debug, PartialEq)]
enum Env {
    Dev,
    Prod,
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Env::Dev),
            "prod" => Ok(Env::Prod),
            _ => Err(format!("Invalid value for enum Env: {}", s)),
        }
    }
}

#[derive(Debug, FromEnv)]
struct Config {
    ip: Ipv4Addr,
    env: Env,
}

fn main() {
    std::env::set_var("IP", "0.0.0.0");
    std::env::set_var("ENV", "Prod");
    std::env::set_var("WEB_IP", "1.1.1.1");
    std::env::set_var("WEB_ENV", "dev");

    let config = Config::from_env_with_prefix("");
    let config2 = Config::from_env_with_prefix("WEB_");

    assert!(config.ip.eq(&Ipv4Addr::new(0, 0, 0, 0)));
    assert!(config.env == Env::Prod);

    assert!(config2.ip.eq(&Ipv4Addr::new(1, 1, 1, 1)));
    assert!(config2.env == Env::Dev);
}
```