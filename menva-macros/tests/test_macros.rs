use menva_macros::FromEnv;

#[derive(Debug)]
enum Env {
    Dev,
    Prod,
}

impl FromStr for S {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "Dev" => Ok(S::Dev),
            "Prod" => Ok(S::Prod),
            _ => Err(format!("Invalid value for enum S: {}", s)),
        }
    }
}

#[derive(Debug, FromEnv)]
struct Config {
    database_url: String,
    api_key: String,
    port: u16,
    env: Env,
}

#[test]
fn test_from_env_macro() {
    std::env::set_var("DATABASE_URL", "postgres://localhost");
    std::env::set_var("API_KEY", "12345");
    std::env::set_var("PORT", "8080");
    std::env::set_var("ENV", "Prod");

    let config = Config::from_env();

    assert!(config.database_url.eq("postgres://localhost"));
    assert!(config.api_key.eq("12345"));
    assert!(config.port.eq(8080));
    assert!(config.env.eq(Env::Prod));
}
