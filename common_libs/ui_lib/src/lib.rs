mod layout;
mod render;
mod util;
pub use crate::{
    layout::{
        RootNode, Node,
        Button, Label,
    },
    render::{
        Colour,
    },
};
pub use glutin::{
    event::{ControlFlow},
    event_loop::{EventLoop},
};

use glutin::{
    ContextBuilder, WindowedContext, PossiblyCurrent,
    dpi::{PhysicalSize},
    event_loop::{EventLoop},
    monitor::{MonitorHandle},
    window::{Fullscreen, WindowBuilder},
};
use crate::{
    render::{
        RenderDef,
    },
};

mod gl_mod {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct UIContext<T> {
    gl: gl::Gl,
    gl_context: WindowedContext<PossiblyCurrent>,
}
impl UIContext {
    pub fn new(options: WindowCreateOptions) -> Result<(UIContext, EventLoop), String> {
        let event_loop = EventLoop::new();
        let window_builder = options.into_window_builder(event_loop.primary_monitor());
        let gl_context = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .map_err(|e| e.to_string())?;
        let gl_context = unsafe { gl_context.make_current() }
            .map_err(|e| e.to_string())?;

        let gl = gl_mod::Gl::load_with(|s| gl_context.get_proc_address(s) as *const _);

        Ok( (UIContext {
            gl,
            gl_context,
            last_render_def: RenderDef::default(),
        }, event_loop) )
    }

    // pub fn handle_events(&self, node_manager: &NodeManager<>)
}

pub struct WindowCreateOptions {
    name: Option<String>,
    size: Option<(u32, u32)>,
    fullscreen: bool,
}
impl WindowCreateOptions {
    pub fn empty() -> WindowCreateOptions {
        WindowCreateOptions {
            name: None,
            size: None,
            fullscreen: false,
        }
    }
}
impl WindowCreateOptions {
    fn into_window_builder(self, primary_monitor: MonitorHandle) -> WindowBuilder {
        let window_builder = WindowBuilder::new();
        let window_builder = self.name.map_or(window_builder,
            |name| window_builder.with_title(name));
        let window_builder = if self.fullscreen {
            window_builder.with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)))
        } else if let Some( (width, height) ) = self.size {
            window_builder.with_inner_size(PhysicalSize::new(width, height))
        };
        window_builder
    }
}
impl Default for WindowCreateOptions {
    fn default() -> Self {
        WindowCreateOptions {
            name: Some("[PH] UI Lib".to_string()),
            size: Some( (1024, 720) ),
            fullscreen: false,
        }
    }
}
