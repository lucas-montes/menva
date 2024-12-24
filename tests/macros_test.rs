use std::net::Ipv4Addr;
use std::str::FromStr;

use menva::FromEnv;

#[test]
fn test_from_env_with_prefix() {
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

#[test]
fn test_from_env_macro() {
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
        database_url: String,
        #[env_prefix("MYAPP_")]
        api_key: String,
        ip: Ipv4Addr,
        port: u16,
        env: Env,
    }

    std::env::set_var("DATABASE_URL", "postgres://localhost");
    std::env::set_var("MYAPP_API_KEY", "12345");
    std::env::set_var("PORT", "8080");
    std::env::set_var("IP", "0.0.0.0");
    std::env::set_var("ENV", "Prod");

    let config = Config::from_env();

    assert!(config.database_url.eq("postgres://localhost"));
    assert!(config.api_key.eq("12345"));
    assert!(config.ip.eq(&Ipv4Addr::new(0, 0, 0, 0)));
    assert!(config.port.eq(&8080));
    assert!(config.env == Env::Prod);
}
