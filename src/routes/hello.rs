use crate::App;
use crate::router::Route;
use crate::routes::Renderer;

pub struct HelloRenderer {
    pub the_line_to_show: String,
}

impl Renderer for HelloRenderer {
    fn render(&self, route: &Route, app: &App) {
        println!("Hello {}", app.name);
    }
}

impl HelloRenderer {
    pub fn new(the_line_to_show: String) -> Self {
        Self {
            the_line_to_show
        }
    }
}