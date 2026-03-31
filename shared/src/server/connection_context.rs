pub struct ConnectionContext {
    pub authenticated: bool,
    pub id: Option<i32>,
    pub ip: String
}

impl ConnectionContext {
    pub fn new(ip: String) -> Self {
        Self {
            authenticated: false,
            id: None,
            ip: ip,
        }
    }
}