mod shapes;
pub use self::shapes::{
    Rectangle, Text,
};

use std::{
    cmp::{Eq, PartialEq},
};
use crate::{
    util,
};

/// A simple RGBA colour
pub struct Colour(f32, f32, f32, f32);
impl Colour {
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Colour {
        let red = util::clamp(red, 0.0, 1.0);
        let green = util::clamp(green, 0.0, 1.0);
        let blue = util::clamp(blue, 0.0, 1.0);
        let alpha = util::clamp(alpha, 0.0, 1.0);
        Colour(red, green, blue, alpha)
    }
}
/// Pre-defined colours
impl Colour {
    // We don't need to clamp these if we don't suck and fail at typing.
    pub fn black() -> Colour { Colour(0.0, 0.0, 0.0, 1.0) }
    pub fn transparent() -> Colour { Colour(0.0, 0.0, 0.0, 0.0) }
    pub fn white() -> Colour { Colour(1.0, 1.0, 1.0, 1.0) }
}

/// Each shape will live in its own layer.
#[derive(Default, Eq, PartialEq)]
pub struct RenderDef {
    shapes: Vec<(Box<RenderableShape>, RenderDetails)>,
}
impl RenderDef {
    pub fn add_shape(&mut self, shape: impl RenderableShape, details: RenderDetails) {
        self.layers.push( (Box::new(shape), details) );
    }
}

#[derive(Default, Copy, Clone)]
pub struct RenderDetails {
    /// In pixels. Will need to be transformed to 3D space in the vertex shader
    position: (f32, f32),
}
impl RenderDetails {
    pub fn with_position(mut self, position: (f32, f32)) -> RenderDetails {
        self.position = position;
        self
    }
}

pub trait RenderableShape: Eq {
    fn render_at(&self, render_details: RenderDetails);
}
