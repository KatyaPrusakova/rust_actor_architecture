use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpFrame {
    pub header: Header,
    pub body: Body
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub method: String,
    pub uri: String
}


#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub chat_id: String,
    pub message_type: MessageType,
    pub single_data: Option<String>,
}
