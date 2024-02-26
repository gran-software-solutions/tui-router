use crate::{App, Renderer};

pub struct Router {
    routes: Vec<Route>,
    current_route: String,
}

pub struct Route {
    pub name: String,
    input_handler: Box<dyn FnMut() -> ()>,
    view: Box<dyn Renderer>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        let route: &Route = &routes[0];
        let name = route.name.clone();
        Router {
            routes,
            current_route: name,
        }
    }

    pub fn handle_input(&self, app: &mut App) {
        app.is_running = false;
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

impl Route {
    pub fn new(
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
