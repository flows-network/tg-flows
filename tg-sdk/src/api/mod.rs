mod method;

use anyhow::Result;
use http_req::{request::Request, uri::Uri};
use serde::de::DeserializeOwned;

use crate::{ChatId, Me, Message, MessageId, True};

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

    pub fn log_out(&self) -> Result<True> {
        self.request(Method::LogOut, &[])
    }

    pub fn close(&self) -> Result<True> {
        self.request(Method::Close, &[])
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

    pub fn forward_message(&self, chat_id: ChatId, from_chat_id: ChatId) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "from_chat_id": from_chat_id,
        });
        self.request(Method::ForwardMessage, body.to_string().as_bytes())
    }

    pub fn copy_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: MessageId,
    ) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "from_chat_id": from_chat_id,
            "message_id": message_id,
        });
        self.request(Method::CopyMessage, body.to_string().as_bytes())
    }

    pub fn edit_message_text<T>(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        text: T,
    ) -> Result<Message>
    where
        T: Into<String>,
    {
        let text = text.into();
        let body = serde_json::json!({
            "chat_id": chat_id,
            "message_id": message_id,
            "text": text,
        });
        self.request(Method::EditMessageText, body.to_string().as_bytes())
    }

    // EditMessageCaption,
    // EditMessageMedia,
    // EditMessageLiveLocation,
    // StopMessageLiveLocation,
    // EditMessageReplyMarkup,
    // StopPoll,
    // DeleteMessage,
}
