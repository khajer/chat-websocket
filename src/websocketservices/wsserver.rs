// use std::collections::HashMap;

use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use actix::{Actor, Message, StreamHandler};
use actix_web_actors::ws::{self};
use rand::Rng;

use crate::websocketservices::session::SessionMessage;

use super::room::Room;

pub struct WSServer {
    sessions: HashMap<usize, Recipient<SessionMessage>>, // <id, receient> like db
    rooms: HashMap<String, Room>,
}

impl WSServer {
    pub fn new() -> WSServer {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        WSServer {
            sessions: HashMap::new(),
            rooms: rooms,
        }
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub name: String,
    pub addr: Recipient<SessionMessage>,
}

#[derive(Message)]
#[rtype(result = "(usize)")]
pub struct Connect {
    pub addr: Recipient<SessionMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

impl Handler<JoinRoom> for WSServer {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        println!("Received: {}", msg.name);

        let msg_out = SessionMessage {
            message: "room name: ".to_string() + &msg.name.to_string(),
        };
    }
}

impl Handler<Connect> for WSServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();

        println!("CONNECT: {}", id);

        self.sessions.insert(id, msg.addr.clone());

        let msg_out = SessionMessage {
            message: format!("connect : {}", id),
        };
        // default to main room
        // self.rooms.entry("main".to_owned()).or_default().insert(id);
        msg.addr.do_send(msg_out);
        id
    }
}

impl Handler<Disconnect> for WSServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        println!("DISCONNECT: {}", msg.id);
        self.sessions.remove(&msg.id);
    }
}
