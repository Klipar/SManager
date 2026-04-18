pub struct ConnectionContext {
    pub authenticated: bool,
    pub id: Option<i64>,
}

impl ConnectionContext {
    pub fn new() -> Self {
        Self {
            authenticated: false,
            id: None,
        }
    }
}