use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Settlement {
    pub id: Uuid,
    pub name: String,
    pub center_x: f32,
    pub center_y: f32,
    pub core_level: u8,
    pub safe_radius: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
}

impl Settlement {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let dx = x - self.center_x;
        let dy = y - self.center_y;
        (dx * dx + dy * dy).sqrt() <= self.safe_radius
    }
}
