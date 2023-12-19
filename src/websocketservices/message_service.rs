use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageInput {
    pub cmd: String,
    pub params: Option<Value>,
}

pub fn parse_message_command(str: &str) -> MessageInput {
    let result: Result<MessageInput> = serde_json::from_str(str);
    match result {
        Ok(message_input) => message_input,
        Err(err) => {
            print!("{}", err);
            MessageInput {
                cmd: "unknown".to_string(),
                params: Some(Value::Null),
            }
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

        assert_eq!(result.cmd, "unknown");
    }

    #[test]
    pub fn test_parse_string_to_struct_emptry_json() {
        let txt_json = r#"{}"#;
        let result = parse_message_command(txt_json);

        assert_eq!(result.cmd, "unknown");
    }

    #[test]
    pub fn test_parse_string_to_struct_json_and_tag_name() {
        let txt_json = r#"{
            cmd:"lobby"
        }"#; //not for match because cmd not 'cmd'

        let result = parse_message_command(txt_json);
        assert_eq!(result.cmd, "unknown");
    }
    #[test]
    pub fn test_parse_string_to_lobby() {
        let txt_json = r#"{
            "cmd":"lobby"
        }"#;

        let result = parse_message_command(txt_json);
        assert_eq!(result.cmd, "lobby");
    }

    #[test]
    pub fn test_parse_string_to_lobby_name() {
        let txt_json = r#"{
            "cmd":"lobby",
            "params":{
                "name":"itsara"
            }
        }"#;

        let result = parse_message_command(txt_json);
        assert_eq!(result.cmd, "lobby");
        assert_eq!(result.params.unwrap()["name"], "itsara");
    }
}
