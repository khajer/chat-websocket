use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Message, StreamHandler};
use actix_web_actors::ws::{self};
use rand::Rng;

use crate::websocketservices::session::SessionMessage;

use super::room::Room;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub name: String,
    pub id: usize,
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

pub struct WSServer {
    sessions: HashMap<usize, Recipient<SessionMessage>>, // <id, receient> like db
    rooms: HashMap<String, Room>,
}

impl WSServer {
    pub fn new() -> WSServer {
        let mut rooms = HashMap::new();
        let mut r = Room::new();
        r.members.clear();
        rooms.insert("main".to_owned(), r);

        WSServer {
            sessions: HashMap::new(),
            rooms,
        }
    }
    fn send_message_room(&mut self, room_name: String, msg: String) {
        if let Some(room) = self.rooms.get(&room_name) {
            for adr_member in room.members.clone().into_iter() {
                if let Some(addr) = self.sessions.get(&adr_member) {
                    let session_message = SessionMessage {
                        message: msg.clone(),
                    };
                    addr.do_send(session_message);
                }
            }
        }

        println!("message room");
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
        self.rooms
            .entry(msg.name.clone())
            .or_insert_with(Room::new)
            .members
            .insert(msg.id);

        self.send_message_room(msg.name.clone(), "people come join".to_string());
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
