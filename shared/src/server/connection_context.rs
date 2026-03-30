pub struct ConnectionContext {
    pub authenticated: bool,
}

impl ConnectionContext {
    pub fn new() -> Self {
        Self {
            authenticated: false,
        }
    }
}