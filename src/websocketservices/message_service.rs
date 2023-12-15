use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    LOBBY,
    JOIN,
    LEFT,
    CHAT,
    UNKNOWN,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MessageInput {
    cmd: String,
}

pub fn parse_message_command(str: &str) -> Message {
    let result: Result<MessageInput> = serde_json::from_str(str);
    match result {
        Ok(message_input) => {
            println!("{}", message_input.cmd);

            match message_input.cmd.as_str() {
                "lobby" => Message::LOBBY,
                "chat" => Message::CHAT,
                "join" => Message::JOIN,
                "left" => Message::LEFT,
                _ => Message::UNKNOWN,
            }
        }
        Err(err) => {
            print!("{}", err);
            Message::UNKNOWN
        }
    }
}

////////////////////////
mod test {
    use super::*;

    #[test]
    pub fn test_parse_string_to_struct_not_json() {
        let txt_json = r#"{"#;
        let result = parse_message_command(txt_json);

        assert_eq!(result, Message::UNKNOWN);
    }

    #[test]
    pub fn test_parse_string_to_struct_emptry_json() {
        let txt_json = r#"{}"#;
        let result = parse_message_command(txt_json);

        assert_eq!(result, Message::UNKNOWN);
    }

    #[test]
    pub fn test_parse_string_to_struct_json_and_tag_name() {
        let txt_json = r#"
    {
        cmd:"xxx"
    }    
    "#;
        parse_message_command(txt_json);
        let result = parse_message_command(txt_json);
        assert_eq!(result, Message::UNKNOWN);
    }
    #[test]
    pub fn test_parse_string_to_lobby() {
        let txt_json = r#"{
            "cmd":"lobby"
        }"#;
        parse_message_command(txt_json);
        let result = parse_message_command(txt_json);
        assert_eq!(result, Message::LOBBY);
    }
    #[test]
    pub fn test_parse_string_to_room() {
        let txt_json = r#"{
            "cmd":"chat"
        }"#;
        parse_message_command(txt_json);
        let result = parse_message_command(txt_json);
        assert_eq!(result, Message::CHAT);
    }
}
