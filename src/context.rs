use crate::{kurbo::Size, tree::Node};

//pub mod element;
//pub mod window;

pub struct ElementCtx {
    pub(crate) size: Size,
    pub(crate) requires_im_pass: bool,
    pub(crate) requires_layout: bool,
    pub(crate) requires_paint: bool,
}
impl ElementCtx {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn request_im_pass(&mut self) {
        self.requires_im_pass = true;
    }

    pub fn request_layout(&mut self) {
        self.requires_layout = true;
    }

    pub fn request_paint(&mut self) {
        self.requires_paint = true;
    }

    pub(crate) fn apply_to_node(&self, node: &mut Node) {
        node.requests.requires_im_pass |= self.requires_im_pass;
        node.requests.requires_layout |= self.requires_layout;
        node.requests.requires_paint |= self.requires_paint;
    }
}
impl From<&Node> for ElementCtx {
    fn from(node: &Node) -> Self {
        ElementCtx {
            size: node.state.size,
            requires_im_pass: false,
            requires_layout: false,
            requires_paint: false,
        }
    }
}
