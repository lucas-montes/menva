use std::net::Ipv4Addr;

#[test]
fn test_from_env_macro() {
    use std::str::FromStr;

    use menva_macros::FromEnv;

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
