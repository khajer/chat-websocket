use actix::{Actor, Message, StreamHandler};
use actix_web_actors::ws::{self};

// #[derive(Message)]
// #[rtype()]
// pub struct LOBBY {
//     pub addr: Recipient<Message>,
// }
#[derive(Message)]
#[rtype(result = "()")]
pub struct LOBBY {
    pub name: String,
}

pub struct WSServer {
    total: u16,
}

impl WSServer {
    pub fn new() -> WSServer {
        WSServer { total: 0 }
    }
}

impl Actor for WSServer {
    type Context = actix::Context<WSServer>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Start Thread Server");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("In comming Server");
    }
}

// impl Handler<LOBBY> for Server {
//     type Result = String;
//     fn handle(&mut self, msg: Message, _: &mut Context<Self>) -> Self::Result {
//         format!("Received: {}", msg.content)
//     }
// }
// impl Handler fro WSServer{

// }
