pub struct Player {
    name: String,
    items: Vec<String>,
}

impl Player {
    pub fn new(name: &str, items: Vec<String>) -> Self {
        Player {
            name: name.to_string(),
            items,
        }
    }
}
