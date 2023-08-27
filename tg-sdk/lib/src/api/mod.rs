mod method;

use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use http_req::{request::Request, uri::Uri};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    Chat, ChatId, ChatInviteLink, ChatMember, ChatPermissions, File, InputFile, Me, Message,
    MessageId, True, UserId, UserProfilePhotos,
};

pub use self::method::Method;

const BASE_URL: &str = "https://api.telegram.org/bot";

pub struct Telegram {
    base_url: &'static str,
    token: String,
}

impl Telegram {
    pub fn new(token: String) -> Self {
        Self {
            base_url: BASE_URL,
            token,
        }
    }

    pub fn new_with_base_url(token: String, base_url: &'static str) -> Self {
        Self { base_url, token }
    }
}

impl Telegram {
    pub fn request<T>(&self, method: Method, body: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}/{}", self.base_url, self.token, method.to_string());
        let uri = Uri::try_from(url.as_str())?;

        let mut writer = Vec::new();
        Request::new(&uri)
            .header("Content-Type", "application/json")
            .header("Content-Length", &body.len())
            .body(body)
            .send(&mut writer)?;

        let value = serde_json::from_str::<Value>(&String::from_utf8(writer)?)?;

        let result = value
            .get("result")
            .ok_or(anyhow!("tg api returned without result"))?;
        let t: T = serde_json::from_value(result.clone())?;

        Ok(t)
    }

    pub fn request_with_attach<T, S>(
        &self,
        method: Method,
        query: HashMap<S, S>,
        body: &[u8],
    ) -> Result<T>
    where
        T: DeserializeOwned,
        S: Serialize,
    {
        let mut url = format!("{}{}/{}", self.base_url, self.token, method.to_string());
        if !query.is_empty() {
            let mut iter = query.iter();
            let (k, v) = iter.next().unwrap();
            let k = serde_json::to_string(k).context("query key serialize error")?;
            let v = serde_json::to_string(v).context("query value serialize error")?;

            url += &format!("?{k}={v}");

            for (k, v) in iter {
                let k = serde_json::to_string(k).context("query key serialize error")?;
                let v = serde_json::to_string(v).context("query value serialize error")?;
                url += &format!("&{k}={v}");
            }
        }

        let uri = Uri::try_from(url.as_str())?;

        let mut writer = Vec::new();
        Request::new(&uri)
            .header("Content-Type", "multipart/form-data")
            .header("Content-Length", &body.len())
            .body(body)
            .send(&mut writer)?;

        let value = serde_json::from_str::<Value>(&String::from_utf8(writer)?)?;

        let result = value
            .get("result")
            .ok_or(anyhow!("tg api returned without result"))?;
        let t: T = serde_json::from_value(result.clone())?;

        Ok(t)
    }
}

// TODO: general & meaningful api
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

    // TODO: multipart request support
    pub fn send_photo(&self, chat_id: ChatId, photo: InputFile) -> Result<Message> {
        if photo.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "photo": photo,
            });
            self.request(Method::SendPhoto, body.to_string().as_bytes())
        }
    }

    pub fn send_audio(&self, chat_id: ChatId, audio: InputFile) -> Result<Message> {
        if audio.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "audio": audio,
            });
            self.request(Method::SendAudio, body.to_string().as_bytes())
        }
    }

    pub fn send_document(&self, chat_id: ChatId, document: InputFile) -> Result<Message> {
        if document.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "document": document,
            });
            self.request(Method::SendDocument, body.to_string().as_bytes())
        }
    }

    pub fn send_video(&self, chat_id: ChatId, video: InputFile) -> Result<Message> {
        if video.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "video": video,
            });
            self.request(Method::SendVideo, body.to_string().as_bytes())
        }
    }

    pub fn send_animation(&self, chat_id: ChatId, animation: InputFile) -> Result<Message> {
        if animation.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "animation": animation,
            });
            self.request(Method::SendAnimation, body.to_string().as_bytes())
        }
    }

    pub fn send_voice(&self, chat_id: ChatId, voice: InputFile) -> Result<Message> {
        if voice.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "voice": voice,
            });
            self.request(Method::SendVoice, body.to_string().as_bytes())
        }
    }

    pub fn send_video_note(&self, chat_id: ChatId, video_note: InputFile) -> Result<Message> {
        if video_note.needs_attach() {
            panic!("unsupport attach currently")
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "video_note": video_note,
            });
            self.request(Method::SendVideoNote, body.to_string().as_bytes())
        }
    }

    // pub fn send_media_group(&self, chat_id: ChatId, media: vec![])

    pub fn send_location(&self, chat_id: ChatId, latitude: f32, longitude: f32) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "latitude": latitude,
            "longitude": longitude,
        });
        self.request(Method::SendLocation, body.to_string().as_bytes())
    }

    pub fn send_venue(
        &self,
        chat_id: ChatId,
        latitude: f32,
        longitude: f32,
        title: String,
        address: String,
    ) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "latitude": latitude,
            "longitude": longitude,
            "title": title,
            "address": address,
        });
        self.request(Method::SendLocation, body.to_string().as_bytes())
    }

    pub fn send_contract(
        &self,
        chat_id: ChatId,
        photo_number: String,
        first_name: String,
    ) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "photo_number":photo_number,
            "first_name": first_name,
        });
        self.request(Method::SendContract, body.to_string().as_bytes())
    }

    pub fn send_poll(
        &self,
        chat_id: ChatId,
        question: String,
        options: Vec<String>,
    ) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "question": question,
            "options": options,
        });
        self.request(Method::SendPoll, body.to_string().as_bytes())
    }

    pub fn send_dice(&self, chat_id: ChatId) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::SendDice, body.to_string().as_bytes())
    }

    pub fn send_chat_action(&self, chat_id: ChatId, action: String) -> Result<Message> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "action": action,
        });
        self.request(Method::SendChatAction, body.to_string().as_bytes())
    }

    pub fn get_user_profile_photos(&self, user_id: UserId) -> Result<UserProfilePhotos> {
        let body = serde_json::json!({
            "user_id": user_id,
        });
        self.request(Method::GetUserProfilePhotos, body.to_string().as_bytes())
    }

    pub fn get_file(&self, file_id: String) -> Result<File> {
        let body = serde_json::json!({
            "file_id": file_id,
        });
        self.request(Method::GetFile, body.to_string().as_bytes())
    }

    pub fn ban_chat_member(&self, chat_id: ChatId, user_id: UserId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::BanChatMember, body.to_string().as_bytes())
    }

    pub fn unban_chat_member(&self, chat_id: ChatId, user_id: UserId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::UnbanChatMember, body.to_string().as_bytes())
    }

    pub fn restrict_chat_member(
        &self,
        chat_id: ChatId,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
            "permissions": permissions,
        });
        self.request(Method::RestrictChatMember, body.to_string().as_bytes())
    }

    pub fn promote_chat_member(&self, chat_id: ChatId, user_id: UserId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::PromoteChatMember, body.to_string().as_bytes())
    }

    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: ChatId,
        user_id: UserId,
        custom_title: String,
    ) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
            "custom_title": custom_title,
        });
        self.request(
            Method::SetChatAdministratorCustomTitle,
            body.to_string().as_bytes(),
        )
    }

    pub fn ban_chat_sender_chat(&self, chat_id: ChatId, sender_chat_id: i32) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "sender_chat_id": sender_chat_id,
        });
        self.request(Method::BanChatSenderChat, body.to_string().as_bytes())
    }

    pub fn unban_chat_sender_chat(&self, chat_id: ChatId, sender_chat_id: i32) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "sender_chat_id": sender_chat_id,
        });
        self.request(Method::UnbanChatSenderChat, body.to_string().as_bytes())
    }

    pub fn set_chat_permissions(
        &self,
        chat_id: ChatId,
        permissions: ChatPermissions,
    ) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "permissions": permissions,
        });
        self.request(Method::SetChatPermissions, body.to_string().as_bytes())
    }

    pub fn export_chat_invite_link(&self, chat_id: ChatId) -> Result<String> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::ExportChatInviteLink, body.to_string().as_bytes())
    }

    pub fn create_chat_invite_link(&self, chat_id: ChatId) -> Result<ChatInviteLink> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::CreateChatInviteLink, body.to_string().as_bytes())
    }

    pub fn edit_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: String,
    ) -> Result<ChatInviteLink> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "invite_link": invite_link,
        });
        self.request(Method::EditChatInviteLink, body.to_string().as_bytes())
    }

    pub fn revoke_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: String,
    ) -> Result<ChatInviteLink> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "invite_link": invite_link,
        });
        self.request(Method::RevokeChatInviteLink, body.to_string().as_bytes())
    }

    pub fn approve_chat_join_request(&self, chat_id: ChatId, user_id: UserId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::ApproveChatJoinRequest, body.to_string().as_bytes())
    }

    pub fn decline_chat_join_request(&self, chat_id: ChatId, user_id: UserId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::DeclineChatJoinRequest, body.to_string().as_bytes())
    }

    pub fn set_chat_photo(&self, chat_id: ChatId, photo: InputFile) -> Result<True> {
        if photo.needs_attach() {
            panic!("unsupport attach currently");
        } else {
            let body = serde_json::json!({
                "chat_id": chat_id,
                "photo": photo,
            });
            self.request(Method::SetChatPhoto, body.to_string().as_bytes())
        }
    }

    pub fn delete_chat_photo(&self, chat_id: ChatId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::DeleteChatPhoto, body.to_string().as_bytes())
    }

    pub fn set_chat_title(&self, chat_id: ChatId, title: String) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "title": title,
        });
        self.request(Method::SetChatTitle, body.to_string().as_bytes())
    }

    pub fn set_chat_description(&self, chat_id: ChatId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::SetChatDescription, body.to_string().as_bytes())
    }

    pub fn pin_chat_message(&self, chat_id: ChatId, message_id: i32) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "message_id": message_id,
        });
        self.request(Method::PinChatMessage, body.to_string().as_bytes())
    }

    pub fn unpin_chat_message(&self, chat_id: ChatId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::UnpinChatMessage, body.to_string().as_bytes())
    }

    pub fn unpin_all_chat_message(&self, chat_id: ChatId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::UnpinAllChatMessages, body.to_string().as_bytes())
    }

    pub fn leave_chat(&self, chat_id: ChatId) -> Result<True> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::LeaveChat, body.to_string().as_bytes())
    }

    pub fn get_chat(&self, chat_id: ChatId) -> Result<Chat> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::GetChat, body.to_string().as_bytes())
    }

    pub fn get_chat_administrators(&self, chat_id: ChatId) -> Result<ChatMember> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::GetChatAdministrators, body.to_string().as_bytes())
    }

    pub fn get_chat_member_count(&self, chat_id: ChatId) -> Result<i32> {
        let body = serde_json::json!({
            "chat_id": chat_id,
        });
        self.request(Method::GetChatMemberCount, body.to_string().as_bytes())
    }

    pub fn get_chat_member(&self, chat_id: ChatId, user_id: UserId) -> Result<ChatMember> {
        let body = serde_json::json!({
            "chat_id": chat_id,
            "user_id": user_id,
        });
        self.request(Method::GetChatMember, body.to_string().as_bytes())
    }

    // TODO: https://core.telegram.org/bots/api#setchatstickerset

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
            "message_id": message_id.0,
            "text": text,
        });
        self.request(Method::EditMessageText, body.to_string().as_bytes())
    }
}
