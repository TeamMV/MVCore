use mvutils::once::CreateOnce;
use mvutils::utils::Recover;
use std::sync::{Arc, RwLock};
use mvutils::unsafe_utils::DangerousCell;

use mvutils::version::Version;

#[cfg(feature = "ui")]
use mvcore::ui::timing::{DurationTask, TimingManager};
use mvcore::{input, ApplicationInfo, MVCore};
use mvcore::render::ApplicationLoopCallbacks;
use mvcore::render::window::{Window, WindowSpecs};

fn main() {
    let core = MVCore::new(ApplicationInfo {
        name: "Test".to_string(),
        version: Version::new(1, 0, 0),
        multithreaded: true,
        extra_threads: 1,
    });
    let mut specs = WindowSpecs::default();
    specs.vsync = false;
    specs.fps = 60;
    specs.decorated = true;
    specs.resizable = true;
    specs.transparent = false;
    specs.width = 800;
    specs.height = 800;
    core.get_render().open_window(specs, ApplicationLoop {});
}

struct ApplicationLoop {}

impl ApplicationLoopCallbacks for ApplicationLoop {
    fn start(&self, window: Arc<Window<Self>>) {

    }

    fn update(&self, window: Arc<Window<Self>>) {}

    fn draw(&self, window: Arc<Window<Self>>) {

    }

    fn effect(&self, window: Arc<Window<Self>>) {

    }

    fn exit(&self, window: Arc<Window<Self>>) {}
}
