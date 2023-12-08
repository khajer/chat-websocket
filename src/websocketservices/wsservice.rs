use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
pub struct MyWs {
    pub clients_vec: Vec<*mut ws::WebsocketContext<MyWs>>,
}
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connected");
        self.clients_vec.push(ctx);
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket stopped");
        let idx = self.clients_vec.iter().position(|&c| c == ctx).unwrap();
        self.clients_vec.remove(idx);
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("message handles>>");

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                println!("byte");
            }
            Ok(ws::Message::Text(text)) => {
                println!("byte message");
                ws_message(ctx, text.to_string());
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

fn ws_message(ctx: &mut ws::WebsocketContext<MyWs>, text: String) {
    println!("receive: '{}'", text);
    if text == "" {
        ctx.text("text1");
    } else {
        ctx.text("text2");
    }
}
