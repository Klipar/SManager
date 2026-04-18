use std::fmt;


#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Endpoint {
    pub ip: String,
    pub port: u16,
}

impl Endpoint {
    pub fn new(ip: impl Into<String>, port: u16) -> Self {
        Self { ip: ip.into(), port }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}