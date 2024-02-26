use crate::app::App;
use crate::router::{Route, Router};
use crate::routes::hello::HelloRenderer;
use crate::routes::Renderer;
use crate::routes::settings::SettingsRenderer;

mod router;
mod routes;
mod app;

fn main() {
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
    let router = Router::new(routes);

    let mut app = App::default();

    while app.is_running {
        router.handle_input(&mut app);
        router.render(&app);
    }
}
