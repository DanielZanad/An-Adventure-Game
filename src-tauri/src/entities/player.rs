use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Player {
    name: String,
    items: Vec<String>,
    pub previous_locations: Vec<String>,
}

impl Player {
    pub fn new(name: &str, items: Vec<String>) -> Self {
        Player {
            name: name.to_string(),
            items,
            previous_locations: vec![],
        }
    }
}
