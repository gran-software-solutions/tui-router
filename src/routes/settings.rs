use crate::App;
use crate::router::Route;
use crate::routes::Renderer;

pub struct SettingsRenderer {
    app_name_input: String, // app.name
}

impl SettingsRenderer {
    pub fn new(app_name_input: String) -> Self {
        Self {
            app_name_input
        }
    }
}

impl Renderer for SettingsRenderer {
    fn render(&self, route: &Route, app: &App) {
        println!("Settings {}", app.name);
    }
}