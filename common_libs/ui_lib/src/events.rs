
#[derive(Default)]
pub struct CallbackMap<T> {
    // on_mouse_move: Option<fn(T, (f32, f32))>,
    on_pointer_up: Option<fn(T)>,
}

pub enum UIEvent {
    // MouseMove(f32, f32),
    PointerUp(f32, f32),
}
