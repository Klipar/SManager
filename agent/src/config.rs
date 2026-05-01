use std::env;

pub struct Config {
    pub database_url: String,
    pub intern_port: u16,
    pub extern_ip: String,
    pub extern_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("AGENT_DATABASE_URL")
                .expect("AGENT_DATABASE_URL must be set"),

            intern_port: Self::parse_u16("AGENT_INTERN_SERVER_PORT", 6767),

            extern_ip: env::var("AGENT_EXTERN_SERVER_IP")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),

            extern_port: Self::parse_u16("AGENT_EXTERN_SERVER_PORT", 6969),
        }
    }

    fn parse_u16(var_name: &str, default: u16) -> u16 {
        env::var(var_name)
            .map(|v| v.parse().unwrap_or(default))
            .unwrap_or(default)
    }
}