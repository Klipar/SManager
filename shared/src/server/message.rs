use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "request")]
    Request {
        id: u64,
        action: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Value>,
    },
    #[serde(rename = "response")]
    Response {
        id: u64,
        status: Status,

        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Value>,
        code: u16,
        message: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
}


impl Message {
    pub fn new_response(
        status: Status,
        data: Option<Value>,
        code: u16,
        message: impl Into<String>
    ) -> Self {
        Message::Response {
            id: 0,
            status,
            data,
            code,
            message: message.into(),
        }
    }

    pub fn set_id(&mut self, new_id: u64) {
        if let Message::Response { id, .. } = self {
            *id = new_id;
        }
    }
}