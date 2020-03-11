#[derive(Debug, Clone)]
pub struct Twitch {
    pub uid: u32,
    pub username: String,
    pub filename: String
}

#[derive(Debug, Clone)]
pub struct Youtube {
    pub channel_id: String,
    pub filename: String
}