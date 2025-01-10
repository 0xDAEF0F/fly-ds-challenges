use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub id: u32,
    pub src: String,
    pub dest: String,
    pub body: ClientBody,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientBody {
    Init(Init),
    Echo(Echo),
    Generate(Generate),
}

#[derive(Debug, Deserialize)]
pub struct Init {
    pub msg_id: u32,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Echo {
    pub msg_id: u32,
    pub echo: String,
}

#[derive(Debug, Deserialize)]
pub struct Generate {
    pub msg_id: u32,
}
