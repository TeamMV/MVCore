use mvutils::once::CreateOnce;
use mvutils::unsafe_utils::DangerousCell;
use mvutils::utils::Recover;
use std::sync::{Arc};

use mvutils::version::Version;
use parking_lot::RwLock;

use mvcore::render::color::RgbColor;
use mvcore::render::common::TextureRegion;
use mvcore::render::window::{Cursor, Window, WindowSpecs};
use mvcore::render::ApplicationLoopCallbacks;
use mvcore::ui::attributes::Attributes;
use mvcore::ui::background::{Background, RoundedBackground};
use mvcore::ui::ease;
use mvcore::ui::ease::Easing;
use mvcore::ui::elements::child::Child;
use mvcore::ui::elements::lmao::LmaoElement;
use mvcore::ui::elements::{UiElement, UiElementCallbacks, UiElementState};
use mvcore::ui::styles::{ChildAlign, Dimension, Direction, Position, TextFit, UiStyle, UiValue};
use mvcore::ui::timing::TIMING_MANAGER;
#[cfg(feature = "ui")]
use mvcore::ui::timing::{DurationTask, TimingManager};
use mvcore::{input, ApplicationInfo, MVCore};

fn main() {
    let core = MVCore::new(ApplicationInfo {
        name: "Test".to_string(),
        version: Version::new(1, 0, 0, 0),
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
    core.get_render().run_window(
        specs,
        ApplicationLoop {
            tex: CreateOnce::new(),
            m: DangerousCell::new(false),
        },
    );
}

struct ApplicationLoop {
    tex: CreateOnce<Arc<TextureRegion>>,
    m: DangerousCell<bool>,
}

impl ApplicationLoopCallbacks for ApplicationLoop {
    fn start(&self, window: Arc<Window<Self>>) {
        self.tex.create(|| {
            Arc::new(TextureRegion::from(Arc::new(
                window.create_texture(include_bytes!("cursor.png").to_vec()),
            )))
        });
        window.set_cursor(Cursor::SoftBusy);
    }

    fn update(&self, window: Arc<Window<Self>>) {}

    fn draw(&self, window: Arc<Window<Self>>) {
        let binding = window.input();
        let input = binding.read().recover();

        let mut style = UiStyle::default();
        style.x.value = UiValue::Just(100);
        style.y.value = UiValue::Just(500);
        style.position = UiValue::Just(Position::Relative);
        style.direction = UiValue::Just(Direction::Vertical);
        style.text.fit = UiValue::Just(TextFit::ExpandParent);
        style.text.size.value = UiValue::Just(100.0);
        style.height.min = UiValue::Just(100);

        let mut elem = LmaoElement::new(Attributes::new(), style.clone());

        let mut elem1 = LmaoElement::new(Attributes::new(), style.clone());
        let mut elem2 = LmaoElement::new(Attributes::new(), style.clone());
        let mut elem3 = LmaoElement::new(Attributes::new(), style);

        elem.add_child(Child::String("Hello1".to_string()));
        elem2.add_child(Child::String("Hello2".to_string()));
        elem3.add_child(Child::String("Hello3".to_string()));

        //elem.add_child(Child::Element(Arc::new(RwLock::new(elem1))));
        //elem.add_child(Child::Element(Arc::new(RwLock::new(elem2))));
        //elem.add_child(Child::Element(Arc::new(RwLock::new(elem3))));

        let mut style = elem.style_mut();
        style.position = UiValue::Just(Position::Absolute);
        style.child_align = UiValue::Just(ChildAlign::Middle);
        //style.width.min = UiValue::Just(500);
        //style.padding.set(UiValue::Just(50));

        let elem = Arc::new(RwLock::new(elem));

        window.draw_2d_pass(|ctx| {
            UiElementState::compute(elem.clone(), ctx);

            let mut guard = elem.write();
            guard.draw(ctx);
        });

        //let mx = input.positions[0];
        //let my = input.positions[1];
        //if input.mouse[input::MOUSE_LEFT] && !self.m.get_val() {
        //    let mut effect = RippleCircleBackgroundEffect::new(RgbColor::white(), 10000, FillMode::Keep, Easing::default());
        //    println!("trigger");
        //    //effect.trigger(
        //    //    Some(TriggerOptions { position: Some(Origin::Custom(mx, my)) }),
        //    //    elem.clone(),
        //    //    window.clone()
        //    //);
        //    *self.m.get_mut() = true;
        //} else {
        //    *self.m.get_mut() = false;
        //}
        //
        unsafe {
            TIMING_MANAGER.post_frame(1.0, 1);//TODO: get window dt and frame
        }
    }

    fn effect(&self, window: Arc<Window<Self>>) {
        //window.enable_effect_2d("wave".to_string());
        //window.enable_effect_2d("pixelate".to_string());
        //window.enable_effect_2d("blur".to_string());
        //window.enable_effect_2d("distort".to_string());
    }

    fn exit(&self, window: Arc<Window<Self>>) {}
}
