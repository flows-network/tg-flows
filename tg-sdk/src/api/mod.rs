mod method;

use anyhow::Result;
use http_req::{request::Request, uri::Uri};
use serde::de::DeserializeOwned;

use crate::{ChatId, Me, Message};

pub use self::method::Method;

const BASE_URL: &str = "https://api.telegram.org/bot";

pub struct Telegram {
    token: String,
}

impl Telegram {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Telegram {
    pub fn request<T>(&self, method: Method, body: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}/{}", BASE_URL, self.token, method.to_string());
        let uri: Uri = Uri::try_from(url.as_str())?;

        let mut writer = Vec::new();
        Request::new(&uri)
            .header("Content-Type", "application/json")
            .header("Content-Length", &body.len())
            .body(body)
            .send(&mut writer)?;

        let value = serde_json::from_str::<T>(&String::from_utf8(writer)?)?;

        Ok(value)
    }
}

impl Telegram {
    pub fn get_me(&self) -> Result<Me> {
        self.request(Method::GetMe, &[])
    }

    pub fn send_message<T>(&self, chat_id: ChatId, text: T) -> Result<Message>
    where
        T: Into<String>,
    {
        let text: String = text.into();
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
        });
        self.request(Method::SendMessage, body.to_string().as_bytes())
    }
}
