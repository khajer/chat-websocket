use serde::{Deserialize, Serialize};
use serde_json::Result;

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

pub fn parse_message_command(text: String) -> Message {
    println!("input : {}", text);

    parse_string_to_struct(text.as_str());

    Message::LOBBY
}

fn parse_string_to_struct(str: &str) {
    let result: Result<MessageInput> = serde_json::from_str(str);
    match result {
        Ok(message_input) => {
            println!("{}", message_input.cmd);
        }
        Err(err) => {
            print!("{}", err)
        }
    }
}

#[cfg(test)]
#[test]
fn test_parse_string_to_struct_normal() {
    let txt_json = r#"
    {
        "cmd":"xxx",
        "name":"xxxx",
        "xxxx":"",

    }    
    "#;
    parse_string_to_struct(txt_json);

    assert_eq!(2 + 2, 4);
}

#[test]
fn test_parse_string_to_struct_emptry_json() {
    let txt_json = r#"
    {}    
    "#;
    parse_string_to_struct(txt_json);
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_parse_string_to_struct_json_and_tag_name() {
    let txt_json = r#"
    {
        cmd:"xxx"
    }    
    "#;
    parse_string_to_struct(txt_json);
    assert_eq!(2 + 2, 4);
}
