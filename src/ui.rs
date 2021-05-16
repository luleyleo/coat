use crate::{
    mutation::TreeMutation,
    tree::{Element, Location, Tree},
};

pub struct Ui<'a> {
    mutation: TreeMutation<'a>,
}
impl<'a> Ui<'a> {
    pub fn new(tree: &'a mut Tree) -> Self {
        let mutation = TreeMutation::new(tree);
        Ui { mutation }
    }

    pub fn add<E, C>(&mut self, location: Location, element: E, content: C)
    where
        E: Element + 'static,
        C: FnOnce(&mut Ui),
    {
        let element = Box::new(element);
        if let Some(node) = self.mutation.next(location) {
            node.element = element;
            content(self);
            self.mutation.end_existing();
        } else {
            self.mutation.insert(location, element);
            content(self);
            self.mutation.end_new();
        }
    }
}
