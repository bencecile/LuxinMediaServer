use std::{
    cmp::{Eq, PartialEq},
};
use super::{
    Colour,
    RenderableShape,
};

/// The first layer is on the bottom.
#[derive(Eq, PartialEq)]
pub struct LayeredShape {
    layers: Vec<(Box<RenderableShape>, (f32, f32))>,
}
impl LayeredShape {
    pub fn new() -> LayeredShape {
        LayeredShape {
            layers: Vec::new(),
        }
    }

    pub fn add_layer(mut self, shape: impl RenderableShape, render_details: RenderDetails) {
        self.layers.push( (Box::new(shape), relative_position) );
        self
    }
}
impl RenderableShape for LayeredShape {
    fn render_at(&self, render_details: RenderDetails) {
    }
}

#[derive(Eq, PartialEq)]
pub struct Rectangle {
    width: f32,
    height: f32,
    fill_colour: Colour,
    rounded_corners: Option<f32>,
    stroke_width: f32,
    stroke_colour: Colour,
}
impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width, height,
            fill_colour: Colour::white(),
            rounded_corners: None,
            stroke_width: 0.0,
            fill_colour: Colour::black(),
        }
    }
    // TODO More builder methods to set the other attributes
}
impl RenderableShape for Rectangle {
    fn render_at(&self, render_details: RenderDetails) {
    }
}

#[derive(Eq, PartialEq)]
pub struct Text {
    text: String,
    text_colour: Colour,
    // font: FontProps,
}
impl Text {
    pub fn new(text: String) -> Text {
        Text {
            text,
            colour: Colour::black(),
        }
    }
}
impl RenderableShape for Text {
    fn render_at(&self, render_details: RenderDetails) {
    }
}
