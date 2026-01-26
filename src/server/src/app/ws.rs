use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::app::AppState;
use crate::world::input::InputEvent;
use crate::world::protocol::{
    validate_input_message, ClientMessage, ServerMessage, SessionSequence, PROTOCOL_VERSION,
};
use crate::world::tick::ServerSnapshot;

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let session_id = Uuid::new_v4();
    let snapshot_rx = state.snapshot_tx().subscribe();
    let input_tx = state.input_tx().clone();

    let session = WsSession::new(session_id, input_tx, snapshot_rx, state.tick_hz());
    ws::start(session, &req, stream)
}

struct WsSession {
    session_id: Uuid,
    input_tx: tokio::sync::mpsc::Sender<InputEvent>,
    snapshot_rx: broadcast::Receiver<ServerSnapshot>,
    seq: SessionSequence,
    tick_hz: u64,
}

impl WsSession {
    fn new(
        session_id: Uuid,
        input_tx: tokio::sync::mpsc::Sender<InputEvent>,
        snapshot_rx: broadcast::Receiver<ServerSnapshot>,
        tick_hz: u64,
    ) -> Self {
        Self {
            session_id,
            input_tx,
            snapshot_rx,
            seq: SessionSequence::default(),
            tick_hz,
        }
    }

    fn send_error(&self, ctx: &mut ws::WebsocketContext<Self>, code: &str, message: &str) {
        let payload = ServerMessage::Error {
            protocol_version: PROTOCOL_VERSION,
            code: code.to_string(),
            message: message.to_string(),
        };
        if let Ok(text) = serde_json::to_string(&payload) {
            ctx.text(text);
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let welcome = ServerMessage::Welcome {
            protocol_version: PROTOCOL_VERSION,
            server_tick: 0,
        };
        if let Ok(text) = serde_json::to_string(&welcome) {
            ctx.text(text);
        }
        let stream = BroadcastStream::new(self.snapshot_rx.resubscribe());
        ctx.add_stream(stream);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(message) => match validate_input_message(message, &mut self.seq) {
                        Ok(validated) => {
                            let event = InputEvent::new(
                                self.session_id,
                                validated.seq,
                                validated.input,
                            );
                            if self.input_tx.try_send(event).is_err() {
                                self.send_error(ctx, "backpressure", "input queue full");
                                ctx.close(None);
                            }
                        }
                        Err(error) => {
                            self.send_error(ctx, "invalid_input", &error.to_string());
                            ctx.close(None);
                        }
                    },
                    Err(err) => {
                        self.send_error(ctx, "invalid_json", &err.to_string());
                        ctx.close(None);
                    }
                }
            }
            Ok(ws::Message::Ping(payload)) => ctx.pong(&payload),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {
                self.send_error(ctx, "unsupported", "binary frames not supported");
                ctx.close(None);
            }
            _ => ctx.stop(),
        }
    }
}

impl StreamHandler<Result<ServerSnapshot, broadcast::error::RecvError>> for WsSession {
    fn handle(
        &mut self,
        item: Result<ServerSnapshot, broadcast::error::RecvError>,
        ctx: &mut Self::Context,
    ) {
        match item {
            Ok(snapshot) => {
                let payload = ServerMessage::Snapshot {
                    protocol_version: PROTOCOL_VERSION,
                    server_tick: snapshot.server_tick,
                };
                if let Ok(text) = serde_json::to_string(&payload) {
                    ctx.text(text);
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => {}
            Err(_) => ctx.stop(),
        }
    }
}
