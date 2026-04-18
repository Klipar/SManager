pub struct ConnectionContext {
    pub authenticated: bool,
    pub id: Option<i32>,
}

impl ConnectionContext {
    pub fn new(_ip: String) -> Self {
        Self {
            authenticated: false,
            id: None,
        }
    }
}