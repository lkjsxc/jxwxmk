#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Resource,
    Mob,
    Structure,
    Npc,
}

impl EntityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityKind::Player => "player",
            EntityKind::Resource => "resource",
            EntityKind::Mob => "mob",
            EntityKind::Structure => "structure",
            EntityKind::Npc => "npc",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub kind: EntityKind,
    pub subtype: String,
    pub x: f64,
    pub y: f64,
    pub hp: f64,
    pub max_hp: f64,
    pub level: i32,
    pub name: Option<String>,
    pub range: Option<f64>,
}

impl Entity {
    pub fn new(id: String, kind: EntityKind, subtype: String, x: f64, y: f64, max_hp: f64) -> Self {
        Self {
            id,
            kind,
            subtype,
            x,
            y,
            hp: max_hp,
            max_hp,
            level: 1,
            name: None,
            range: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.hp = (self.hp - damage).max(0.0);
    }

    pub fn heal(&mut self, amount: f64) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn distance_to(&self, other: &Entity) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn distance_to_point(&self, x: f64, y: f64) -> f64 {
        let dx = self.x - x;
        let dy = self.y - y;
        (dx * dx + dy * dy).sqrt()
    }
}
