use crate::{
    tree::mutation::TreeMutation,
    tree::{Element, Location, Tree},
};

pub struct Ui<'a> {
    mutation: TreeMutation<'a>,
}

impl<'a> Ui<'a> {
    pub fn new(tree: &'a mut Tree) -> Self {
        Ui {
            mutation: TreeMutation::new(tree),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Entry;

    #[test]
    fn single_element() {
        let mut tree = Tree::default();
        let mut ui = Ui::new(&mut tree);

        crate::elements::button(&mut ui, crate::piet::Color::RED);

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());
    }
}
