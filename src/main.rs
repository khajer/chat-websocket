use std::sync::Mutex;

use actix_web::{
    get,
    web::{self, Data},
    Error, HttpRequest, HttpResponse, Responder, Result,
};
use actix_web_actors::ws;
use serde::Serialize;
use websocketservices::rooms::RoomMgr;

mod websocketservices;

#[derive(Serialize)]
struct Version {
    version: String,
}

#[get("/version")]
async fn version() -> Result<impl Responder> {
    let version = Version {
        version: "0.0.1".to_string(),
    };
    Ok(web::Json(version))
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: Data<Mutex<RoomMgr>>,
) -> Result<HttpResponse, Error> {
    let mut r = data.lock().unwrap();
    r.show();
    let resp = ws::start(websocketservices::wsservice::MyWs::new(), &req, stream);

    // println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    let r_mgr = websocketservices::rooms::new();
    let data = Data::new(Mutex::new(r_mgr));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(version)
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
