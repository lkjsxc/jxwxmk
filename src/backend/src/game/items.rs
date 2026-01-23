#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub stackable: bool,
    pub max_stack: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Resource,
    Tool,
    Weapon,
    Armor,
    Food,
    Building,
}

impl Item {
    pub fn new(id: &str, name: &str, item_type: ItemType) -> Self {
        Item {
            id: id.to_string(),
            name: name.to_string(),
            item_type,
            stackable: true,
            max_stack: 64,
        }
    }
}