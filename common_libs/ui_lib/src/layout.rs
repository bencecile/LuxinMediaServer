use std::{
    collections::{BTreeMap},
};

pub struct Node {
    node_type: NodeType,
    params: LayoutParams,
}
impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {
            node_type,
            params: LayoutParams::default(),
        }
    }

    pub fn with_margin(mut self, margin: (f32, f32, f32, f32)) -> Node {
        self.params.margin = margin;
        self
    }
}

pub enum NodeType {
    Label,
    Button,
}


// We should try not to use a position at all (just relative positioning)
#[derive(Default, Copy, Clone)]
pub struct LayoutParams {
    /// Top, right, bottom, left. These are all percents between 0.0 (0%) - 1.0 (100%).
    /// This is the margin to either the parent or another Node, whichever comes first
    margin: (f32, f32, f32, f32),
}

pub trait NodeID: Ord, Eq {
}

#[derive(Default)]
pub struct NodeList<I: NodeID> {
    nodes: BTreeMap<I, (Node, usize)>,
    callbacks: BTreeMap<I, CallbackMap<T>>,
}
impl <I: NodeID> NodeList<I> {
    pub fn add_node(mut self, node: Node, id: I) -> NodeList {
        self.nodes.insert(id, node);
        self
    }

    pub fn add_callbacks(mut self, callbacks: CallbackMap<T>) -> NodeList {
        self.callbacks.insert()
    }
}

type UIPosition = (f32, f32);

pub struct NodeManager<I: NodeID> {
    nodes: BTreeMap<I, NodeWrapper>,
}
impl <I: NodeID> NodeManager<I> {
    pub fn from_node_list(node_list: NodeList<I>) -> NodeManager<I> {
    }

    /// Returns true if there was a change that requires a new render
    pub fn merge_with_manager(&mut self, new_manager: NodeManager<I>) -> bool {
    }
}
/// Other properties that will need to be known about a node
struct NodeWrapper {
    node: Node,
    z_order: usize,
    // Top-left and bottom-right coords
    rectangle: (UIPosition, UIPosition),
    // TODO Might want to cache vertices or textures in here
}


// use crate::{
//     render::{
//         RenderDef,
//         Rectangle, Text,
//     },
// };

// type BoxedNodes = ;

// pub struct RootNode {
//     nodes: Vec<Box<Node>>,
// }
// impl RootNode {
//     pub fn new() -> RootNode {
//         RootNode {
//             children: Vec::new(),
//         }
//     }

//     #[inline]
//     pub fn add_child(mut self, child: impl Node) -> Self {
//         self.children.push(Box::new(child));
//         self
//     }
// }

// // TODO Give some screen details to this function
// pub fn render_root_node(root_node: RootNode) -> RenderDef {
//     let mut render_def = RenderDef::default();
//     // TODO We're going to have to get pixel positions and lengths for every node
//     for node in root_node.nodes {
//         render_def.add_shape(node.render());
//     }

//     render_def
// }

// pub trait Node {
//     fn params(&mut self) -> &mut LayoutParams;
//     fn id(&self) -> &str;
//     fn render(self) -> impl RenderableShape;
//     // TODO A method that returns a list of callbacks with what kind of event triggers it
// }

// pub struct Label {
//     label: String,
//     layout_params: LayoutParams,
// }
// impl Label {
//     pub fn new(label: String) -> Label {
//         Label {
//             label,
//             layout_params: LayoutParams::new(),
//         }
//     }
// }
// impl Node for Label {
//     fn params(&mut self) -> &mut LayoutParams { &mut self.params }
//     fn render(self) -> impl RenderableShape {
//     }
// }

// pub struct Button {
//     face: ButtonFace,
//     layout_params: LayoutParams,
// }
// impl Button {
//     pub fn new_with_label(label: Label) -> Button {
//         Button {
//             face: ButtonFace::Label(label),
//             layout_params: LayoutParams,
//         }
//     }
// }
// impl Node for Button {
//     fn params(&mut self) -> &mut LayoutParams { &mut self.params }
//     fn render(self) -> impl RenderableShape {
//     }
// }

// enum ButtonFace {
//     Label(Label),
// }
