use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub chat: Chat,
    pub from: User,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
}

pub struct TeleApi {
    token: String,
    client: Client,
}

impl TeleApi {
    pub fn new(token: &str) -> TeleApi {
        TeleApi {
            token: token.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, Error> {
        let method = "getUpdates?offset=-1";
        let response = self
            .client
            .post(&format!("https://api.telegram.org/bot{}/{}", self.token, method))
            .send()
            .await?;
        let updates: Vec<Update> = response.json().await?;
        Ok(updates)
    }

    pub async fn execute(&self, method: &str, params: &[(&str, &str)]) -> Result<(), anyhow::Error> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, method);

        let mut form_params = params.to_owned();
        form_params.push(("method", method));

        let response = self
            .client
            .post(&url)
            .form(&form_params)
            .send()
            .await?;

        let response_text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response_text)?;

        if json["ok"].as_bool().unwrap_or(false) {
            Ok(())
        } else {
            Err(anyhow!(
                "Telegram API Error: {}",
                json["description"].as_str().unwrap_or("Unknown error")
            ))
        }
    }
}

macro_rules! auto_generate {
    ($($method:ident),*) => {
        $(
            impl TeleApi {
                pub async fn $method(&self, params: &[(&str, &str)]) -> Result<(), anyhow::Error> {
                    let method = stringify!($method);
                    self.execute(method, params).await
                }
            }
        )*
    };
}

auto_generate!(
    sendMessage,
    sendPhoto,
    sendAudio,
    sendVideo,
    sendDice,
    sendLocation,
    sendDocument,
    sendContact,
    editMessage,
    deleteWebhook,
    getWebhookInfo,
    ReplyKeyboardMarkup,
    KeyboardButton,
    WebAppInfo,
    ReplyKeyboardRemove,
    KeyboardButtonRequestChat,
    CallbackQuery,
    SwitchInlineQueryChosenChat,
    ForceReply,
    ChatPhoto,
    ChatInviteLink,
    ChatAdministratorRights,
    ChatMember,
    ChatMemberOwner,
    ChatMemberAdministrator,
    ChatMemberMember,
    ChatMemberRestricted,
    ChatMemberLeft,
    ChatMemberBanned,
    ChatMemberUpdated,
    ChatJoinRequest,
    ChatPermissions,
    ForumTopic,
    BotCommand,
    BotCommandScope,
    forwardMessage,
    copyMessage,
    sendAnimation,
    sendVoice,
    sendVideoNote,
    sendMediaGroup,
    sendVenue,
    sendPoll,
    sendChatAction,
    getUserProfilePhotos,
    getFile,
    banChatMember,
    unbanChatMember,
    restrictChatMember,
    promoteChatMember,
    setChatAdministratorCustomTitle,
    banChatSenderChat,
    unbanChatSenderChat,
    setChatPermissions,
    exportChatInviteLink,
    createChatInviteLink,
    editChatInviteLink,
    revokeChatInviteLink,
    approveChatJoinRequest,
    declineChatJoinRequest,
    setChatPhoto,
    deleteChatPhoto,
    setChatTitle,
    setChatDescription,
    pinChatMessage,
    unpinChatMessage,
    unpinAllChatMessages,
    leaveChat,
    getChat,
    getChatAdministrators,
    getChatMemberCount,
    getChatMember,
    setChatStickerSet,
    deleteChatStickerSet,
    getForumTopicIconStickers,
    createForumTopic,
    editForumTopic,
    closeForumTopic,
    reopenForumTopic,
    deleteForumTopic,
    unpinAllForumTopicMessages,
    editGeneralForumTopic,
    closeGeneralForumTopic,
    reopenGeneralForumTopic,
    hideGeneralForumTopic,
    unhideGeneralForumTopic,
    unpinAllGeneralForumTopicMessages,
    answerCallbackQuery,
    setMyCommands,
    deleteMyCommands,
    getMyCommands,
    getMyName,
    setMyName,
    getMyDescription,
    setMyDescription,
    setMyShortDescription,
    getMyShortDescription,
    setChatMenuButton,
    getChatMenuButton,
    setMyDefaultAdministratorRights,
    getMyDefaultAdministratorRights,
    editMessageText,
    editMessageCaption,
    editMessageMedia,
    editMessageLiveLocation,
    stopMessageLiveLocation,
    editMessageReplyMarkup,
    stopPoll,
    deleteMessage,
    Sticker,
    StickerSet,
    MaskPosition,
    InputSticker,
    sendSticker,
    getStickerSet,
    getCustomEmojiStickers,
    uploadStickerFile,
    createNewStickerSet,
    addStickerToSet,
    setStickerPositionInSet,
    deleteStickerFromSet,
    setStickerEmojiList,
    setStickerKeywords,
    setStickerMaskPosition,
    setStickerSetTitle,
    setStickerSetThumbnail,
    setCustomEmojiStickerSetThumbnail,
    deleteStickerSet,
    InlineQuery,
    answerInlineQuery,
    InlineQueryResultsButton,
    InlineQueryResultArticle,
    InlineQueryResultPhoto,
    InlineQueryResultGif,
    InlineQueryResultMpeg4Gif,
    InlineQueryResultVideo,
    InlineQueryResultAudio,
    InlineQueryResultVoice,
    InlineQueryResultDocument,
    InlineQueryResultLocation,
    InlineQueryResultVenue,
    InlineQueryResultGame,
    InlineQueryResultContact,
    InlineQueryResultCachedPhoto,
    InlineQueryResultCachedGif,
    InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedSticker,
    InlineQueryResultCachedDocument,
    InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice,
    InlineQueryResultCachedAudio,
    InputMessageContent,
    InputTextMessageContent,
    InputLocationMessageContent,
    InputVenueMessageContent,
    InputContactMessageContent,
    InputInvoiceMessageContent,
    ChosenInlineResult,
    answerWebAppQuery,
    SentWebAppMessage,
    sendInvoice,
    createInvoiceLink,
    answerShippingQuery,
    answerPreCheckoutQuery,
    LabeledPrice,
    Invoice,
    ShippingAddress,
    OrderInfo,
    ShippingOption,
    SuccessfulPayment,
    PreCheckoutQuery,
    ShippingQuery,
    PassportData,
    PassportFile,
    EncryptedPassportElement,
    EncryptedCredentials,
    setPassportDataErrors,
    PassportElementError,
    PassportElementErrorDataField,
    PassportElementErrorFrontSide,
    PassportElementErrorReverseSide,
    PassportElementErrorSelfie,
    PassportElementErrorFile,
    PassportElementErrorFiles,
    PassportElementErrorTranslationFile,
    PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
    sendGame,
    Game,
    CallbackGame,
    setGameScore,
    getGameHighScores,
    GameHighScore
);
