pub struct ConnectionContext {
    pub id: Option<i32>,
    pub ip: String
}

impl ConnectionContext {
    pub fn new(ip: String) -> Self {
        Self {
            id: None,
            ip: ip,
        }
    }
}