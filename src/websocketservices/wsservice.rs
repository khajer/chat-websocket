use std::collections::HashMap;

use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub struct MyWs {
    pub lobby_players: HashMap<String, *mut ws::WebsocketContext<MyWs>>,
}
impl MyWs {
    fn receive_message(&mut self, ctx: &mut ws::WebsocketContext<MyWs>, text: String) {
        let name = "".to_string();

        self.lobby_players.insert(name, ctx);
        ctx.text(text.to_string());
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
                // println!("byte");
            }
            Ok(ws::Message::Text(text)) => {
                // println!("byte message");
                self.receive_message(ctx, text.to_string());
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Binary");
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                println!("Close");
                // let idx = self.clients_vec.iter().position(|&c| c == ctx).unwrap();
                // self.clients_vec.remove(idx);
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

fn testing() -> bool {
    true
}
#[cfg(test)]
#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn check() {
    assert_eq!(testing(), true);
}
