use std::sync::mpsc::Receiver;
use crossterm::event::KeyCode;
use crate::{App, Renderer};
use crate::tui::TerminalEvent;

pub struct Router {
    routes: Vec<Route>,
    current_route: String,
    receiver: Receiver<TerminalEvent>,
}

pub struct Route {
    pub name: String,
    input_handler: Box<dyn FnMut() -> ()>,
    view: Box<dyn Renderer>,
}

impl Router {
    pub fn new(routes: Vec<Route>, receiver: Receiver<TerminalEvent>) -> Self {
        let route: &Route = &routes[0];
        let name = route.name.clone();
        Router {
            routes,
            current_route: name,
            receiver,
        }
    }

    pub fn handle_input(&self, app: &mut App) {
        let event = self.receiver.recv().expect("could not read TerminalEvent");

        match event {
            TerminalEvent::Key(e) => {
                if let KeyCode::Esc = e.code {
                    app.is_running = false;
                }
            }
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
