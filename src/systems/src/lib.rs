use protocol::Vec2;
use world::Entity;

pub fn update_movement(entity: &mut Entity, dx: f32, dy: f32, speed: f32, dt: f32) {
    let mag = (dx * dx + dy * dy).sqrt();
    if mag > 0.0 {
        entity.pos.x += (dx / mag) * speed * dt;
        entity.pos.y += (dy / mag) * speed * dt;
    }
}
