// TODO: with optional params

use strum::Display;

#[derive(Debug, Display)]
#[strum(serialize_all = "camelCase")]
pub enum Method {
    GetMe,
    LogOut,
    Close,
    SendMessage,
    ForwardMessage,
    CopyMessage,
    SendPhoto,
    SendAudio,
    SendDocument,
    SendVideo,
    SendAnimation,
    SendVoice,
    SendVideoNote,
    SendMediaGroup,
    SendLocation,
    SendVenue,
    SendContract,
    SendPoll,
    SendDice,
    SendChatAction,
    GetUserProfilePhotos,
    GetFile,
    BanChatMember,
    UnbanChatMember,
    RestrictChatMember,
    PromoteChatMember,
    SetChatAdministratorCustomTitle,
    BanChatSenderChat,
    UnbanChatSenderChat,
    SetChatPermissions,
    ExportChatInviteLink,
    CreateChatInviteLink,
    EditChatInviteLink,
    RevokeChatInviteLink,
    ApproveChatJoinRequest,
    DeclineChatJoinRequest,
    SetChatPhoto,
    DeleteChatPhoto,
    SetChatTitle,
    SetChatDescription,
    PinChatMessage,
    UnpinChatMessage,
    UnpinAllChatMessages,
    LeaveChat,
    GetChat,
    GetChatAdministrators,
    GetChatMemberCount,
    GetChatMember,
    SetChatStickerSet,
    DeleteChatStickerSet,
    GetForumTopicIconStickers,
    CreateForumTopic,
    EditForumTopic,
    CloseForumTopic,
    ReopenForumTopic,
    DeleteForumTopic,
    UnpinAllForumTopicMessages,
    EditGeneralForumTopic,
    CloseGeneralForumTopic,
    ReopenGeneralForumTopic,
    HideGeneralForumTopic,
    UnhideGeneralForumTopic,
    AnswerCallbackQuery,
    SetMyCommands,
    DeleteMyCommands,
    GetMyCommands,
    SetMyDescription,
    GetMyDescription,
    SetMyShortDescription,
    GetMyShortDescription,
    SetChatMenuButton,
    GetChatMenuButton,
    SetMyDefaultAdministratorRights,
    GetMyDefaultAdministratorRights,
    EditMessageText,
    EditMessageCaption,
    EditMessageMedia,
    EditMessageLiveLocation,
    StopMessageLiveLocation,
    EditMessageReplyMarkup,
    StopPoll,
    DeleteMessage,
}

#[cfg(test)]
mod tests {
    use crate::Method;

    #[test]
    fn test_display() {
        assert_eq!(
            String::from("sendMessage"),
            format!("{}", Method::SendMessage)
        );
        assert_eq!(
            String::from("editMessageMedia"),
            format!("{}", Method::EditMessageMedia),
        );
        assert_eq!(
            String::from("closeGeneralForumTopic"),
            format!("{}", Method::CloseGeneralForumTopic),
        );
    }
}
