pub mod settings {
    use config::{Config, Environment, File};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Default)]
    pub struct Server {
        pub port: i32,
    }

    #[derive(Debug, Deserialize, Default)]
    pub struct Logging {
        pub log_level: String,
    }

    #[derive(Debug, Deserialize, Default)]
    pub struct Settings {
        pub database: Server,
        pub logging: Logging,
    }

    impl Settings {
        pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
            let s = Config::builder()
                .add_source(File::with_name(location))
                .add_source(Environment::with_prefix(env_prefix).separator("__").prefix_separator("__"));
            // s.add_source(File::with_name(location).required(true))
            //     .add_source(Environment::with_prefix(env_prefix).separator("__").prefix_separator("__"));
            let settings = s.try_deserialize()?;
            Ok(settings)
        }
    }
}