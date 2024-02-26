use crate::App;
use crate::router::Route;

pub mod hello;
pub mod settings;

pub trait Renderer {
    fn render(&self, route: &Route, app: &App);
}
