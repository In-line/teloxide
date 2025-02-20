use serde::{Deserialize, Serialize};

use crate::types::{Message, User};

/// This object represents an incoming callback query from a callback button in
/// an [inline keyboard].
///
/// If the button that originated the query was attached to a message sent by
/// the bot, the field message will be present. If the button was attached to a
/// message sent via the bot (in [inline mode]), the field `inline_message_id`
/// will be present. Exactly one of the fields data or `game_short_name` will be
/// present.
///
/// [The official docs](https://core.telegram.org/bots/api#callbackquery).
///
/// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
/// [inline mode]: https://core.telegram.org/bots/api#inline-mode
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackQuery {
    /// An unique identifier for this query.
    pub id: String,

    /// A sender.
    pub from: User,

    /// A message with the callback button that originated the query. Note that
    /// message content and message date will not be available if the message
    /// is too old.
    pub message: Option<Message>,

    /// An identifier of the message sent via the bot in inline mode, that
    /// originated the query.
    pub inline_message_id: Option<String>,

    /// A global identifier, uniquely corresponding to the chat to which the
    /// message with the callback button was sent. Useful for high scores in
    /// [games].
    ///
    /// [games]: https://core.telegram.org/bots/api#games
    pub chat_instance: String,

    /// A data associated with the callback button. Be aware that a bad client
    /// can send arbitrary data in this field.
    pub data: Option<String>,

    /// A short name of a Game to be returned, serves as the unique identifier
    /// for the game.
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    pub fn new<S1, S2>(id: S1, from: User, chat_instance: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            from,
            message: None,
            inline_message_id: None,
            chat_instance: chat_instance.into(),
            data: None,
            game_short_name: None,
        }
    }

    pub fn id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.id = id.into();
        self
    }

    pub fn from(mut self, val: User) -> Self {
        self.from = val;
        self
    }

    pub fn message(mut self, val: Message) -> Self {
        self.message = Some(val);
        self
    }

    pub fn inline_message_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.inline_message_id = Some(val.into());
        self
    }

    pub fn chat_instance<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.chat_instance = val.into();
        self
    }

    pub fn data<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.data = Some(val.into());
        self
    }

    pub fn game_short_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.game_short_name = Some(val.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":"id",
            "from":{
                "id":12345,
                "is_bot":false,
                "first_name":"firstName"
            },
            "inline_message_id":"i_m_id",
            "chat_instance":"123456",
            "data":"some_data",
            "game_short_name":"game_name"
        }"#;
        let expected = CallbackQuery {
            id: "id".to_string(),
            from: User {
                id: 12345,
                is_bot: false,
                first_name: "firstName".to_string(),
                last_name: None,
                username: None,
                language_code: None,
            },
            chat_instance: "123456".to_string(),
            message: None,
            inline_message_id: Some("i_m_id".to_string()),
            data: Some("some_data".to_string()),
            game_short_name: Some("game_name".to_string()),
        };
        let actual = serde_json::from_str::<CallbackQuery>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
