use crate::State;

pub struct View {
    pub root: Node,
}

#[derive(Default)]
pub struct Node {
    pub margin: (Length, Length, Length, Length),
    pub content: Content,
}

pub enum Content {
    Layout(Layout),
    Text(String),
    Empty,
}

#[derive(Default)]
pub struct Layout {
    layout: Vec<Partition>,
    node_lists: Vec<Vec<Node>>,
}

#[derive(Default)]
pub struct Partition {
    offset: Length,
    children: usize,
    vertical: bool,
}

#[derive(Default)]
pub struct Length {
    percent: f32,
    absolute: f32,
}

impl Default for Content {
    fn default() -> Self {
        Content::Empty
    }
}

impl View {
    pub fn new(state: &State) -> Self {
        View {
            root: Node {
                margin: Default::default(),
                content: Content::Text("Hello world".to_string()),
            }
        }
    }
}