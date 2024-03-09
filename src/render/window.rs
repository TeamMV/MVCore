use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::SystemTime;
use mvutils::once::CreateOnce;
use mvutils::unsafe_utils::DangerousCell;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{CursorIcon, Fullscreen, Theme, WindowBuilder};
use crate::input::InputCollector;
use crate::input::raw::Input;
use crate::render::ApplicationLoopCallbacks;
use crate::render::state::State;

pub struct WindowSpecs {
    /// The width of the window in pixels.
    ///
    /// Default is 800.
    pub width: u32,

    /// The height of the window in pixels.
    ///
    /// Default is 600.
    pub height: u32,

    /// The window title, which is displayed at the top of the window.
    ///
    /// Default is an empty string.
    pub title: String,

    /// Whether the window should be fullscreen.
    ///
    /// Default is false.
    pub fullscreen: bool,

    /// Whether the window should have a frame and buttons (like close, minimize and maximize)
    ///
    /// Default is true.
    pub decorated: bool,

    /// Whether the window should be resizable.
    ///
    /// Default is true.
    pub resizable: bool,

    /// Whether the window background is transparent.
    ///
    /// Default is false.
    pub transparent: bool,

    /// Dark or Light theme. None means system theme.
    ///
    /// Default is None.
    pub theme: Option<Theme>,

    /// Whether the window should reduce power consumption at the expense of worse performance by selecting an inferior GPU.
    ///
    /// Default is false.
    pub green_eco_mode: bool,

    /// Whether to sync the screen update with the time the vertical electron beam of your monitor reaches its lowest point.
    ///
    /// Default is true.
    pub vsync: bool,

    /// The maximum framerate of the window.
    ///
    /// Default is 60.
    pub fps: u32,

    /// The maximum update rate of the window.
    ///
    /// Default is 30.
    pub ups: u32,
}

impl Default for WindowSpecs {
    fn default() -> Self {
        WindowSpecs {
            width: 800,
            height: 600,
            title: String::new(),
            fullscreen: false,
            decorated: true,
            resizable: true,
            transparent: false,
            theme: None,
            green_eco_mode: false,
            vsync: true,
            fps: 60,
            ups: 30,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum OperateState {
    Loading,
    Running,
    Paused,
}

pub struct Window<ApplicationLoop: ApplicationLoopCallbacks + 'static> {
    pub specs: DangerousCell<WindowSpecs>,
    start_time: SystemTime,
    state: DangerousCell<State>,

    close: AtomicBool,

    frame: AtomicU64,
    fps: AtomicU64,

    application_loop: ApplicationLoop,
    operate_state: RwLock<OperateState>,
    load_fn: fn(Arc<Window<ApplicationLoop>>),
    // #[cfg(feature = "3d")]
    // model_loader: CreateOnce<ModelLoader<ApplicationLoop>>,

    // camera_2d: RwLock<Camera2D>,
    // camera_3d: RwLock<Camera3D>,

    // draw_2d: Mutex<DrawContext2D>,

    input_collector: DangerousCell<InputCollector>,
    cursor: DangerousCell<Cursor>,
    prev_cursor: DangerousCell<Cursor>,
    internal_window: DangerousCell<winit::window::Window>,
}

unsafe impl<T: ApplicationLoopCallbacks> Send for Window<T> {}

unsafe impl<T: ApplicationLoopCallbacks> Sync for Window<T> {}

impl<T: ApplicationLoopCallbacks + 'static> Window<T> {
    /// Starts the window loop, be aware that this function only finishes when the window is closed or terminated!
    pub fn run(mut specs: WindowSpecs, application_loop: T) {
        let event_loop = EventLoop::new().expect("Failed to create EventLoop");
        let internal_window = WindowBuilder::new()
            .with_transparent(specs.transparent)
            .with_decorations(specs.decorated)
            .with_fullscreen(
                specs
                    .fullscreen.then_some(Fullscreen::Borderless(None)),
            )
            .with_resizable(specs.resizable)
            .with_theme(specs.theme)
            .with_title(specs.title.as_str())
            .with_inner_size(Size::Physical(PhysicalSize::new(specs.width, specs.height)))
            .with_visible(false)
            .build(&event_loop)
            .expect("Failed to open window");

        specs.width = internal_window.inner_size().width;
        specs.height = internal_window.inner_size().height;

        let state = State::new(&internal_window);

        let input_collector = InputCollector::new(RwLock::new(Input::new()).into());

        let mut window = Window {
            specs: specs.into(),
            start_time: SystemTime::now(),
            state: state.into(),
            close: AtomicBool::new(false),
            frame: AtomicU64::new(0),
            fps: AtomicU64::new(0),
            application_loop,
            operate_state: OperateState::Loading.into(),
            load_fn: |_| {},
            input_collector: input_collector.into(),
            cursor: Cursor::None.into(),
            prev_cursor: Cursor::None.into(),
            internal_window: internal_window.into(),
        };
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Cursor {
    None,
    Busy,
    SoftBusy,
    ResizeX,
    ResizeY,
    ResizeXY,
    Move,
    Pointer,
    Denied,
    Text,

    Crosshair,
    VerticalText,
    Cell,
    Copy,
    Grab,
    ZoomIn,
    ZoomOut,
}

impl Cursor {
    pub(crate) fn map_to_winit(&self) -> CursorIcon {
        match self {
            Cursor::None => CursorIcon::Default,
            Cursor::Busy => CursorIcon::Wait,
            Cursor::SoftBusy => CursorIcon::Progress,
            Cursor::ResizeX => CursorIcon::EwResize,
            Cursor::ResizeY => CursorIcon::NsResize,
            Cursor::ResizeXY => CursorIcon::NwseResize,
            Cursor::Move => CursorIcon::Move,
            Cursor::Pointer => CursorIcon::Pointer,
            Cursor::Denied => CursorIcon::NotAllowed,
            Cursor::Text => CursorIcon::Text,

            Cursor::Crosshair => CursorIcon::Crosshair,
            Cursor::VerticalText => CursorIcon::VerticalText,
            Cursor::Cell => CursorIcon::Cell,
            Cursor::Copy => CursorIcon::Copy,
            Cursor::Grab => CursorIcon::Grab,
            Cursor::ZoomIn => CursorIcon::ZoomIn,
            Cursor::ZoomOut => CursorIcon::ZoomOut,
        }
    }
}