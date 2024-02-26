pub struct App {
    pub name: String,
    pub is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            name: "World".to_string(),
            is_running: true,
        }
    }
}
