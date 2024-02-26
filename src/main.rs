use crate::app::App;
use crate::router::{Route, Router};
use crate::routes::hello::HelloRenderer;
use crate::routes::Renderer;
use crate::routes::settings::SettingsRenderer;
use crate::tui::Tui;

mod router;
mod routes;
mod app;
mod tui;

fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();
    
    let tui = Tui::new(sender);
    let routes = vec![
        Route::new(
            "/hello".to_string(),
            Box::new(move || {}),
            Box::new(HelloRenderer::new("Welcome to the Jungle".to_string())),
        ),
        Route::new(
            "/settings".to_string(),
            Box::new(move || {}),
            Box::new(SettingsRenderer::new("Welcome to the Jungle".to_string()))),
    ];
    let router = Router::new(routes, receiver);

    let mut app = App::default();

    tui.enter();
    while app.is_running {
        router.handle_input(&mut app);
        router.render(&app);
    }
    Tui::exit();
}
