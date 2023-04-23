extern crate alloc;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use include_dir::{Dir, include_dir};

use crate::assets::{AssetManager, SemiAutomaticAssetManager, WritableAssetManager};
use crate::render::{RenderCore, RenderingBackend};

pub mod assets;
pub mod render;
pub mod parser;

pub struct MVCore {
    assets: Rc<RefCell<SemiAutomaticAssetManager>>,
    render: Rc<RenderCore>,
}

impl MVCore {
    pub fn new(backend: RenderingBackend) -> MVCore {
        static DIR: Dir = include_dir!("assets");
        let assets = Rc::new(RefCell::new(AssetManager::semi_automatic(DIR.clone())));
        let render = Rc::new(RenderCore::new(backend, assets.clone()));
        assets.borrow_mut().set_render_core(render.clone());
        MVCore {
            assets,
            render
        }.tmp_load()
    }

    pub fn tmp_load(self) -> Self {
        self.assets.borrow_mut().load_shader("default", "shaders/default.vert", "shaders/default.frag");
        self.assets.borrow_mut().load_effect_shader("blur", "shaders/blur.frag");
        self.assets.borrow_mut().load_effect_shader("pixelate", "shaders/pixelate.frag");
        self
    }

    pub fn get_render(&self) -> Rc<RenderCore> {
        self.render.clone()
    }

    pub fn get_asset_manager(&self) -> Ref<SemiAutomaticAssetManager> {
        self.assets.borrow()
    }

    pub fn get_asset_manager_mut(&mut self) -> RefMut<SemiAutomaticAssetManager> {
        self.assets.borrow_mut()
    }

    pub fn terminate(mut self) {
        self.term();
        drop(self);
    }

    fn term(&mut self) {
        self.render.terminate();
    }
}

impl Drop for MVCore {
    fn drop(&mut self) {
        self.term();
    }
}

impl Default for MVCore {
    fn default() -> Self {
        Self::new(RenderingBackend::OpenGL)
    }
}

#[cfg(test)]
#[cfg(not(feature = "vulkan"))]
mod tests {
    use crate::assets::ReadableAssetManager;
    use crate::MVCore;
    use crate::render::RenderingBackend::OpenGL;
    use crate::render::shared::*;

    #[test]
    fn test() {
        let mut core = MVCore::new(OpenGL);
        let mut info = WindowCreateInfo::default();
        info.title = "MVCore".to_string();
        let mut window = core.get_render().create_window(info);
        window.add_shader("blur", core.get_asset_manager().get_effect_shader("blur"));
        window.add_shader("pixelate", core.get_asset_manager().get_effect_shader("pixelate"));
        window.run(Test);
    }

    struct Test;

    impl ApplicationLoop for Test {
        fn start(&self, window: &mut impl Window) {}

        fn update(&self, window: &mut impl Window) {}

        fn draw(&self, window: &mut impl Window) {
            window.get_draw_2d().tri();
            //window.queue_shader_pass(ShaderPassInfo::new("pixelate", |shader| {
            //  shader.uniform_1f("size", 10.0);
            //}));
            //window.queue_shader_pass(ShaderPassInfo::new("blur", |shader| {
            //    shader.uniform_1f("dir", 16.0);
            //    shader.uniform_1f("quality", 4.0);
            //    shader.uniform_1f("size", 8.0);
            //}));
        }

        fn stop(&self, window: &mut impl Window) {}
    }
}

#[cfg(test)]
#[cfg(feature = "vulkan")]
mod vulkan_test {


    #[test]
    fn test() {

    }

}
