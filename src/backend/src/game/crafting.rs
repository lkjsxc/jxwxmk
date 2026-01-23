use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CraftingRecipe {
    pub id: String,
    pub name: String,
    pub result: String,
    pub ingredients: HashMap<String, u32>,
    pub crafting_time: f32,
}

impl CraftingRecipe {
    pub fn new(id: &str, name: &str, result: &str) -> Self {
        CraftingRecipe {
            id: id.to_string(),
            name: name.to_string(),
            result: result.to_string(),
            ingredients: HashMap::new(),
            crafting_time: 1.0,
        }
    }

    pub fn add_ingredient(&mut self, item_id: &str, quantity: u32) {
        self.ingredients.insert(item_id.to_string(), quantity);
    }
}