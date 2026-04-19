pub struct ConnectionContext {
    pub id: Option<i32>,
    pub ip: String,
    pub user_id: Option<i32>,
    pub is_admin: bool,
}

impl ConnectionContext {
    pub fn new(ip: String) -> Self {
        Self {
            id: None,
            ip: ip,
            user_id: None,
            is_admin: false,
        }
    }
}