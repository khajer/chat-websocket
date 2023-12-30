// use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Message, StreamHandler};
use actix_web_actors::ws::{self};

use crate::websocketservices::session::SessionMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub name: String,
    pub addr: Recipient<SessionMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DISCONNECT {
    pub name: String,
    // pub addr: Addr<Session>,
    pub addr: Recipient<SessionMessage>,
}

pub struct WSServer {
    // sessions: HashMap<usize, Recipient<Message>>,
    // rooms: HashMap<String, HashSet<usize>>,
}

impl WSServer {
    pub fn new() -> WSServer {
        WSServer {}
    }
}

impl Actor for WSServer {
    type Context = actix::Context<WSServer>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Start Thread Server");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSServer {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        println!("In comming Server");
    }
}

impl Handler<JoinRoom> for WSServer {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        println!("Received: {}", msg.name);

        let msg_out = SessionMessage {
            message: "room name: ".to_string() + &msg.name.to_string(),
        };
        msg.addr.do_send(msg_out);
    }
}

impl Handler<DISCONNECT> for WSServer {
    type Result = ();
    fn handle(&mut self, msg: DISCONNECT, _: &mut Context<Self>) -> Self::Result {
        println!("DISCONNECT: {}", msg.name);
    }
}
