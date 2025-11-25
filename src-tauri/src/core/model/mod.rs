use std::sync::Arc;

use super::renderer::Renderer;

mod live2d;

pub trait Model {
    fn bind_renderer(&mut self, renderer: Arc<Renderer>);
    fn draw(&mut self);
}