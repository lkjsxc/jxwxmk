use actix::{Actor, StreamHandler, AsyncContext, Handler, Addr, ActorContext, Running, WrapFuture, ActorFutureExt, ContextFutureSpawner};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::game::engine::{GameEngine, Join, Leave, Input, ServerMessage, Craft, SelectSlot, UpdateName, SwapSlots, Spawn, NpcAction, Trade, AcceptQuest};
use crate::game::entities::item::{Item, ItemType};
use serde::{Serialize, Deserialize};

pub struct GameSession { pub id: Uuid, pub token: Option<String>, pub game_engine: Addr<GameEngine> }

impl GameSession {
    pub fn new(engine: Addr<GameEngine>, token: Option<String>) -> Self {
        Self { id: Uuid::new_v4(), token, game_engine: engine }
    }
}

#[derive(Serialize)] struct WelcomeMsg { #[serde(rename = "type")] msg_type: String, id: Uuid, token: String, spawned: bool }
#[derive(Serialize)] struct WorldMsg<T> { #[serde(rename = "type")] msg_type: String, data: T }
#[derive(Serialize)] struct NotificationPayload { title: String, message: String, color: String }
#[derive(Serialize)] struct NpcInteractionPayload { 
    npc_id: Uuid, npc_type: crate::game::entities::npc::NpcType, name: String, text: String, options: Vec<String>, trade_items: Option<Vec<Item>> 
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().recipient();
        let join_msg = Join { id: self.id, token: self.token.clone(), addr };
        self.game_engine.send(join_msg).into_actor(self).then(|res, act, ctx: &mut ws::WebsocketContext<GameSession>| {
            if let Ok(Some((token, id, spawned))) = res {
                act.id = id;
                let welcome = WelcomeMsg { msg_type: "welcome".to_string(), id, token, spawned };
                if let Ok(json) = serde_json::to_string(&welcome) { ctx.text(json); }
            } else { ctx.stop(); }
            actix::fut::ready(())
        }).wait(ctx);
    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running { self.game_engine.do_send(Leave { id: self.id }); Running::Stop }
}

#[derive(Deserialize)]
struct ClientMessage {
    #[serde(default)] dx: f64, #[serde(default)] dy: f64, #[serde(default)] attack: bool, #[serde(default)] interact: bool,
    #[serde(default)] craft: Option<ItemType>, #[serde(default)] slot: Option<usize>, #[serde(default)] name: Option<String>,
    #[serde(default)] swapSlots: Option<(usize, usize)>, #[serde(default)] spawn: bool,
    #[serde(default)] npcAction: Option<(Uuid, u32)>, #[serde(default)] trade: Option<(Uuid, usize, bool)>,
    #[serde(default)] acceptQuest: Option<String>,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            if let Ok(m) = serde_json::from_str::<ClientMessage>(&text) {
                if m.spawn { self.game_engine.do_send(Spawn { id: self.id }); }
                if let Some(n) = m.name { self.game_engine.do_send(UpdateName { id: self.id, name: n }); }
                if let Some(s) = m.slot { self.game_engine.do_send(SelectSlot { id: self.id, slot: s }); }
                if let Some(c) = m.craft { self.game_engine.do_send(Craft { id: self.id, item: c }); }
                if let Some((f, t)) = m.swapSlots { self.game_engine.do_send(SwapSlots { id: self.id, from: f, to: t }); }
                if let Some((npc_id, opt)) = m.npcAction { self.game_engine.do_send(NpcAction { id: self.id, npc_id, option_index: opt }); }
                if let Some((npc_id, idx, buy)) = m.trade { self.game_engine.do_send(Trade { id: self.id, npc_id, item_index: idx, buy }); }
                if let Some(qid) = m.acceptQuest { self.game_engine.do_send(AcceptQuest { id: self.id, quest_id: qid }); }
                self.game_engine.do_send(Input { id: self.id, dx: m.dx, dy: m.dy, attack: m.attack, interact: m.interact });
            }
        } else if let Ok(ws::Message::Ping(msg)) = msg { ctx.pong(&msg); }
    }
}

impl Handler<ServerMessage> for GameSession {
    type Result = ();
    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        match msg {
            ServerMessage::WorldUpdate(world) => {
                let wrapper = WorldMsg { msg_type: "world".to_string(), data: world };
                if let Ok(json) = serde_json::to_string(&wrapper) { ctx.text(json); }
            }
            ServerMessage::AchievementUnlocked(ach) => {
                let wrapper = WorldMsg { msg_type: "achievement".to_string(), data: ach };
                if let Ok(json) = serde_json::to_string(&wrapper) { ctx.text(json); }
            }
            ServerMessage::Notification { title, message, color } => {
                let payload = NotificationPayload { title, message, color };
                let wrapper = WorldMsg { msg_type: "notification".to_string(), data: payload };
                if let Ok(json) = serde_json::to_string(&wrapper) { ctx.text(json); }
            }
            ServerMessage::NpcInteraction { npc_id, npc_type, name, text, options, trade_items } => {
                let payload = NpcInteractionPayload { npc_id, npc_type, name, text, options, trade_items };
                let wrapper = WorldMsg { msg_type: "npcInteraction".to_string(), data: payload };
                if let Ok(json) = serde_json::to_string(&wrapper) { ctx.text(json); }
            }
            ServerMessage::QuestUpdate(quest) => {
                let wrapper = WorldMsg { msg_type: "questUpdate".to_string(), data: quest };
                if let Ok(json) = serde_json::to_string(&wrapper) { ctx.text(json); }
            }
        }
    }
}

#[derive(Deserialize)] pub struct WsQuery { token: Option<String> }
pub async fn ws_index(r: HttpRequest, stream: web::Payload, engine: web::Data<Addr<GameEngine>>, q: web::Query<WsQuery>) -> Result<HttpResponse, Error> {
    ws::start(GameSession::new(engine.get_ref().clone(), q.token.clone()), &r, stream)
}