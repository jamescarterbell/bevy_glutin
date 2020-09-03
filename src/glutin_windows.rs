use bevy_utils::HashMap;
use bevy_window::{Window, WindowId, WindowMode};
use glutin::window::WindowBuilder;
#[derive(Default)]
pub struct GlutinContexts{
    contexts: HashMap<glutin::window::WindowId, glutin::ContextWrapper<glutin::NotCurrent, ()>>,
    window_id_to_glutin: HashMap<WindowId, glutin::window::WindowId>,
    glutin_to_window_id: HashMap<glutin::window::WindowId, WindowId>,
}

impl GlutinContexts{
    fn create_context(&mut self, el: &glutin::event_loop::EventLoopWindowTarget<()>, wb: WindowBuilder, bw: &Window) -> glutin::window::Window{
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();
        let (context, window) = unsafe{windowed_context.split()};
        
        self.window_id_to_glutin.insert(bw.id, window.id());
        self.glutin_to_window_id.insert(window.id(), bw.id);

        self.contexts.insert(window.id(), context);
        window
    }

    pub fn get_context(&self, id: WindowId) -> Option<&glutin::ContextWrapper<glutin::NotCurrent, ()>> {
        self.window_id_to_glutin
            .get(&id)
            .and_then(|id| self.contexts.get(id))
    }

    pub fn take_context(&mut self, id: WindowId) -> Option<glutin::ContextWrapper<glutin::NotCurrent, ()>>{
        self.window_id_to_glutin
            .remove(&id)
            .and_then(|id| {
                self.glutin_to_window_id.remove(&id);
                self.contexts.remove(&id)
            })
    }

    pub fn get_window_id(&self, id: glutin::window::WindowId) -> Option<WindowId> {
        self.glutin_to_window_id.get(&id).cloned()
    }
}

unsafe impl Send for GlutinContexts{}
unsafe impl Sync for GlutinContexts{}

#[derive(Default)]
pub struct GlutinWindows {
    pub windows: HashMap<glutin::window::WindowId, glutin::window::Window>,
    pub window_id_to_glutin: HashMap<WindowId, glutin::window::WindowId>,
    pub glutin_to_window_id: HashMap<glutin::window::WindowId, WindowId>,
}

impl GlutinWindows {
    pub fn create_window(
        &mut self,
        event_loop: &glutin::event_loop::EventLoopWindowTarget<()>,
        window: &Window,
        contexts: &mut GlutinContexts,
    ) {
        #[cfg(target_os = "windows")]
        let mut glutin_window_builder = {
            glutin::window::WindowBuilder::new()
        };

        #[cfg(not(target_os = "windows"))]
        let mut glutin_window_builder = glutin::window::WindowBuilder::new();

        glutin_window_builder = match window.mode {
            WindowMode::BorderlessFullscreen => glutin_window_builder.with_fullscreen(Some(
                glutin::window::Fullscreen::Borderless(event_loop.primary_monitor()),
            )),
            WindowMode::Fullscreen { use_size } => glutin_window_builder.with_fullscreen(Some(
                glutin::window::Fullscreen::Exclusive(match use_size {
                    true => get_fitting_videomode(&event_loop.primary_monitor(), &window),
                    false => get_best_videomode(&event_loop.primary_monitor()),
                }),
            )),
            _ => glutin_window_builder
                .with_inner_size(glutin::dpi::PhysicalSize::new(window.width, window.height))
                .with_resizable(window.resizable),
        };

        let glutin_window = contexts.create_context(
            &event_loop,
            glutin_window_builder
                    .with_title(&window.title),
            window);
            
        self.window_id_to_glutin.insert(window.id, glutin_window.id());
        self.glutin_to_window_id.insert(glutin_window.id(), window.id);

        self.windows.insert(glutin_window.id(), glutin_window);
    }

    pub fn get_window(&self, id: WindowId) -> Option<&glutin::window::Window> {
        self.window_id_to_glutin
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }

    pub fn get_window_id(&self, id: glutin::window::WindowId) -> Option<WindowId> {
        self.glutin_to_window_id.get(&id).cloned()
    }
}
fn get_fitting_videomode(
    monitor: &glutin::monitor::MonitorHandle,
    window: &Window,
) -> glutin::monitor::VideoMode {
    let mut modes = monitor.video_modes().collect::<Vec<_>>();

    fn abs_diff(a: u32, b: u32) -> u32 {
        if a > b {
            return a - b;
        }
        b - a
    }

    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match abs_diff(a.size().width, window.width).cmp(&abs_diff(b.size().width, window.width)) {
            Equal => {
                match abs_diff(a.size().height, window.height)
                    .cmp(&abs_diff(b.size().height, window.height))
                {
                    Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                    default => default,
                }
            }
            default => default,
        }
    });

    modes.first().unwrap().clone()
}

fn get_best_videomode(monitor: &glutin::monitor::MonitorHandle) -> glutin::monitor::VideoMode {
    let mut modes = monitor.video_modes().collect::<Vec<_>>();
    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match b.size().width.cmp(&a.size().width) {
            Equal => match b.size().height.cmp(&a.size().height) {
                Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                default => default,
            },
            default => default,
        }
    });

    modes.first().unwrap().clone()
}
