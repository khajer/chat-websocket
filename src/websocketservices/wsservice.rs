use super::message_service;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use std::collections::HashMap;

pub struct MyWs {
    pub lobby_players: HashMap<String, *mut ws::WebsocketContext<MyWs>>,
}

impl MyWs {
    fn receive_message(&mut self, ctx: &mut ws::WebsocketContext<MyWs>, text: String) {
        let msg_input = message_service::parse_message_command(text.as_str());
        match msg_input.cmd.as_str() {
            "lobby" => {
                let params = msg_input.params.unwrap();
                println!("name login : {}", params["name"]);
                self.assign_to_lobby(params["name"].to_string(), ctx);
            }
            _ => {
                println!("unknown cmd ");
            }
        };
    }
    fn assign_to_lobby(&mut self, name: String, ctx: &mut ws::WebsocketContext<MyWs>) {
        println!("{} assign to lobby", name);
        self.lobby_players.insert(name, ctx);
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                self.receive_message(ctx, text.to_string());
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                // let idx = self.clients_vec.iter().position(|&c| c == ctx).unwrap();
                // self.clients_vec.remove(idx);
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

// fn testing() -> bool {
//     true
// }
// #[cfg(test)]
// #[test]
// fn exploration() {
//     assert_eq!(2 + 2, 4);
// }

// #[test]
// fn check() {
//     assert_eq!(testing(), true);
// }
