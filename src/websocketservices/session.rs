use std::time::{Duration, Instant};

use crate::websocketservices::wsserver::{ChatMessage, JoinRoom, Name};

use super::wsserver::{Connect, Disconnect};
use super::{message_service, wsserver::WSServer};
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use rand::Rng;
use sha2::{Digest, Sha256};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Session {
    pub hb: Instant,
    pub id: usize,
    pub name: String,
    pub room: String,
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
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
    fn receive_message(&mut self, _ctx: &mut ws::WebsocketContext<Session>, text: String) {
        println!("[session_id:{}][message]: {}", self.id, text);
        let msg_input = message_service::parse_message_command(text.as_str());
        match msg_input.cmd.as_str() {
            "name" => {
                let params = msg_input.params.unwrap();
                println!("set name = {}", params["name"].to_string());
                self.name = params["name"].to_string();
                self.addr.do_send(Name {
                    name: params["name"].to_string(),
                });
            }
            "join" => {
                println!("join room");
                let params = msg_input.params.unwrap();
                let msg = JoinRoom {
                    name: params["name"].to_string(),
                    id: self.id,
                };
                self.addr.do_send(msg);
            }
            "chat" => {
                let params = msg_input.params.unwrap();
                let msg = ChatMessage {
                    message: params["message"].to_string(),
                };
                self.addr.do_send(msg);
                println!("{:}", params);
            }
            _ => {
                println!("unknown cmd");
            }
        };
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.addr
            .send(Connect {
                addr: ctx.address().recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        let msg = Disconnect { id: self.id };
        self.addr.do_send(msg);

        Running::Stop
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
                ctx.close(reason);
            }
            _ => (),
        }
    }
}
