use std::collections::HashMap;

use shared::server::generate_token::generate_token;

pub struct TokenManager {
    token_storage: HashMap<String, i64>
}

impl TokenManager {
    pub fn new() -> Self {
        TokenManager { token_storage: HashMap::new() }
    }

    pub fn gen_token(&mut self, id: i64) -> String{
        let token = generate_token();

        self.token_storage.insert(token.clone(), id);
        return token;
    }

    pub fn use_token(&mut self, token: &String) -> Option<i64>{
        self.token_storage.remove(token)
    }
}