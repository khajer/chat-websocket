use crate::websocketservices::wsserver::LOBBY;

use super::{message_service, wsserver::WSServer};
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Session {
    pub session_id: String,
    pub name: String,
    pub addr: Addr<WSServer>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SessionMessage {
    pub message: String,
}
impl Handler<SessionMessage> for Session {
    type Result = ();
    fn handle(
        &mut self,
        msg: SessionMessage,
        ctx: &mut ws::WebsocketContext<Self>,
    ) -> Self::Result {
        println!("session message: {}", msg.message);
        ctx.text(msg.message);
    }
}

impl Session {
    fn receive_message(&mut self, ctx: &mut ws::WebsocketContext<Session>, text: String) {
        println!("[session_id:{}][message]: {}", self.session_id, text);
        let msg_input = message_service::parse_message_command(text.as_str());
        match msg_input.cmd.as_str() {
            "lobby" => {
                let params = msg_input.params.unwrap();
                self.name = params["name"].to_string();
                let msg = LOBBY {
                    name: "kha".to_string(),
                    addr: ctx.address(),
                };
                self.addr.do_send(msg);
                println!(
                    "name login : {}, session_id : {}",
                    self.name, self.session_id
                );
            }
            "chat" => {
                let params = msg_input.params.unwrap();
                println!("{:}", params);
            }
            _ => {
                println!("unknown cmd ");
            }
        };
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        self.session_id = generate_session_id();
        println!(
            "WebSocket connection started with session ID: {}",
            self.session_id
        );
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
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
fn generate_session_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen();

    let mut hasher = Sha256::new();
    hasher.update(random_number.to_string());
    let hash_result = hasher.finalize();

    format!("{:x}", hash_result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_session_id_test() {
        println!("session_id: {}", generate_session_id());
        assert_eq!(3, 3);
    }
}
