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
