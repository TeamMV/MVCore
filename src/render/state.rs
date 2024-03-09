use filament_bindings::backend::Backend;
use filament_bindings::filament::{Engine, Renderer, SwapChain, SwapChainConfig, Viewport};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};


pub struct State {
    engine: Engine,
    surface: *mut std::ffi::c_void,
    swapchain: SwapChain,
    renderer: Renderer,
    viewport: Viewport,
    aspect: f64,
}

impl State {
    pub(crate) fn new(window: &winit::window::Window) -> Self {
        unsafe {
            let mut engine = Engine::create(Backend::DEFAULT).expect("Failed to create filament engine");
            let surface = {
                let handle = window.window_handle().expect("Failed to get window handle").as_raw();
                match handle {
                    RawWindowHandle::UiKit(handle) => handle.ui_view.as_ptr(),
                    RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr(),
                    RawWindowHandle::Orbital(handle) => handle.window.as_ptr(),
                    RawWindowHandle::Xlib(handle) => handle.window as *mut std::ffi::c_void,
                    RawWindowHandle::Xcb(handle) => handle.window.get() as *mut std::ffi::c_void,
                    RawWindowHandle::Wayland(handle) => handle.surface.as_ptr(),
                    RawWindowHandle::Drm(handle) => handle.plane as *mut std::ffi::c_void,
                    RawWindowHandle::Gbm(handle) => handle.gbm_surface.as_ptr(),
                    RawWindowHandle::Win32(handle) => handle.hwnd.get() as *mut std::ffi::c_void,
                    RawWindowHandle::WinRt(handle) => handle.core_window.as_ptr(),
                    RawWindowHandle::AndroidNdk(handle) => handle.a_native_window.as_ptr(),
                    RawWindowHandle::Haiku(handle) => handle.b_window.as_ptr(),
                    _ => unreachable!()
                }
            };

            let swapchain = engine.create_swap_chain(surface, SwapChainConfig::default()).expect("Failed to create filament swapchain");

            let renderer = engine.create_renderer().unwrap();

            let viewport = Viewport {
                left: 0,
                bottom: 0,
                width: window.inner_size().width,
                height: window.inner_size().height,
            };

            let aspect = window.inner_size().width as f64 / window.inner_size().height as f64;

            State {
                engine,
                surface,
                swapchain,
                renderer,
                viewport,
                aspect,
            }
        }
    }
}