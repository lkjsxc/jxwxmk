use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::prelude::*;
use std::time::{Duration, Instant};

use crate::models::player::Player;
use crate::services::player_service::PlayerService;

pub struct MyWebSocket {
    player_id: Option<uuid::Uuid>,
    last_heartbeat: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self {
            player_id: None,
            last_heartbeat: Instant::now(),
        }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.last_heartbeat = Instant::now();
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > Duration::from_secs(10) {
                ctx.stop();
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(MyWebSocket::new(), &req, stream)?;
    Ok(resp)
}