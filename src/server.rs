use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServerMessage {
    pub src: String,
    pub dest: String,
    pub body: ServerBody,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerBody {
    InitOk(Init),
    EchoOk(Echo),
    GenerateOk(Generate),
}

#[derive(Debug, Serialize)]
pub struct Init {
    pub in_reply_to: u32,
}

#[derive(Debug, Serialize)]
pub struct Echo {
    pub in_reply_to: u32,
    pub msg_id: u32,
    pub echo: String,
}

#[derive(Debug, Serialize)]
pub struct Generate {
    pub id: String,
    pub in_reply_to: u32,
}
