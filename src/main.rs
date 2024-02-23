struct Router {
    routes: Vec<Route>,
    current_route: String,
}

impl Router {
    pub fn handle_input(&self, app: &mut App) {
        app.is_running = false;
    }
}

impl Router {
    fn new(routes: Vec<Route>) -> Self {
        let route: &Route = &routes[0];
        let name = route.name.clone();
        Router {
            routes,
            current_route: name,
        }
    }

    pub fn render(&self, app: &App) {
        self.get_current_route().render(app);
    }

    fn get_current_route(&self) -> &Route {
        self.routes.iter().find(|r| {
            r.name == self.current_route
        }).expect("No way that there's no route")
    }
}

struct Route {
    pub name: String,
    input_handler: Box<dyn FnMut() -> ()>,
    view: Box<dyn Renderer>,
}

impl Route {
    fn new(
        name: String,
        input_handler: Box<dyn FnMut() -> ()>,
        view: Box<dyn Renderer>) -> Self {
        Self {
            name,
            input_handler,
            view,
        }
    }

    pub fn render(&self, app: &App) {
        self.view.render(self, app);
    }
}

struct App {
    name: String,
    is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            name: "World".to_string(),
            is_running: true,
        }
    }
}

trait Renderer {
    fn render(&self, route: &Route, app: &App);
}

struct HelloRenderer {
    the_line_to_show: String,
}

impl Renderer for HelloRenderer {
    fn render(&self, route: &Route, app: &App) {
        println!("Hello {}", app.name);
    }
}

impl HelloRenderer {
    fn new(the_line_to_show: String) -> Self {
        Self {
            the_line_to_show
        }
    }
}

struct SettingsRenderer {
    app_name_input: String, // app.name
}

impl SettingsRenderer {
    fn new(app_name_input: String) -> Self {
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
