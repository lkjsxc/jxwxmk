use actix::{Actor, StreamHandler, AsyncContext, Handler, Message};
use actix_web_actors::ws;
use protocol::{ClientMessage, ServerMessage};
use game::{EngineEvent, GameEngine};
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct GameSession {
    pub id: Uuid,
    pub engine_tx: mpsc::UnboundedSender<EngineEvent>,
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
        
        // Notify engine
        let _ = self.engine_tx.send(EngineEvent::PlayerJoined(self.id, tx));

        // Listen for messages from engine to send to client
        let session_id = self.id;
        ctx.add_stream(async_stream::stream! {
            while let Some(msg) = rx.recv().await {
                yield msg;
            }
        }.map(|msg| WsServerMessage(msg)));
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct WsServerMessage(ServerMessage);

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    let _ = self.engine_tx.send(EngineEvent::ClientMsg(self.id, client_msg));
                }
            }
            Ok(ws::Message::Close(reason)) => {
                let _ = self.engine_tx.send(EngineEvent::PlayerLeft(self.id));
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl StreamHandler<WsServerMessage> for GameSession {
    fn handle(&mut self, msg: WsServerMessage, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg.0) {
            ctx.text(json);
        }
    }
}

use futures_util::StreamExt;
use actix::ActorContext;
