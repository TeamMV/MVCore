pub(crate) mod state;
pub mod window;

use std::sync::Arc;
use crate::render::window::{Window, WindowSpecs};

pub struct RenderCore {}

impl RenderCore {
    pub(crate) fn new() -> Arc<Self> {
        RenderCore {}.into()
    }

    pub fn open_window<T: ApplicationLoopCallbacks + 'static>(&self, specs: WindowSpecs, application_loop: T) {
        Window::run(specs, application_loop)
    }
}

pub trait ApplicationLoopCallbacks: Sized {
    fn start(&self, window: Arc<Window<Self>>);
    fn update(&self, window: Arc<Window<Self>>);
    fn draw(&self, window: Arc<Window<Self>>);
    fn effect(&self, window: Arc<Window<Self>>);
    fn exit(&self, window: Arc<Window<Self>>);
}