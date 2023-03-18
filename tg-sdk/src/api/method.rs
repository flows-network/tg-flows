#[derive(Debug)]
pub enum Method {
    GetMe,
    SendMessage,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::GetMe => String::from("getMe"),
            Self::SendMessage => String::from("sendMessage"),
        }
    }
}
