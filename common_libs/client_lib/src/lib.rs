use async_std::{
    sync::{Arc, Mutex},
    task,
};
use ui_lib::{
    UIContext, RootNode,
    EventLoop, ControlFlow,
};

type ArcMutex<T> = Arc<Mutex<T>>;
// type AppCallback<T: AppData> = fn(app_data: &mut T) -> Option<>;

pub struct App<T> {
    app_data: ArcMutex<T>,
    current: ArcMutex<dyn AppState<AppData = T>>,
    previous: Vec<ArcMutex<dyn AppState<AppData = T>>>,
    // TODO Have a list of ongoing async tasks
}
impl <T> App<T> {
    pub fn new(app_data: T, app_state: impl AppState<AppData = T>) -> App<T> {
        App {
            app_data: Arc::new(Mutex::new(app_state)),
            previous: Vec::new(),
        }
    }

    pub fn run(mut self, context: UIContext, event_loop: EventLoop) -> ! {
        event_loop.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { window_event, .. } => match window_event {
                    // WindowEvent::Resized(physical_size) => {
                    //     windowed_context.resize(physical_size)
                    // }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    // windowed_context.swap_buffers().unwrap();
                },
                _ => (),
            }
        });
    }
}

pub trait AppState {
    type AppData;

    type NodeID;
    fn layout(&self, app_data: &Self::AppData) -> RootNode<Self::NodeID>;

    fn on_start(&mut self, context: &AppContext<Self::AppData>) {}
    fn on_resume(&mut self, context: &AppContext<Self::AppData>) {}
    fn on_pause(&mut self, context: &AppContext<Self::AppData>) {}
    fn on_end(&mut self, context: &AppContext<Self::AppData>) {}
}

#[derive(Clone)]
pub struct AppContext<T> {
    data: ArcMutex<T>,
    // TODO Be able to reference the main App
}
impl <T> AppContext<T> {
    // TODO Use this context to spawn async tasks. It might want access to the AppState it got called with
    // TODO May want to put state transition functions in here
    pub fn modify_data<F>(&self, fun: F) where F: FnOnce(&mut T) {
        let mut lock = task::block_on(self.data.lock()).unwrap();
        fun(&mut lock)
    }
}
