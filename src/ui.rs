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

    pub fn add<E, U, C>(&mut self, location: Location, update: U, content: C)
    where
        E: Element + Default + 'static,
        U: FnOnce(&mut E),
        C: FnOnce(&mut Ui),
    {
        if let Some(node) = self.mutation.next(location) {
            update(node.element.as_mut_any().downcast_mut::<E>().unwrap());
            content(self);
            self.mutation.end_existing();
        } else {
            let mut element = E::default();
            update(&mut element);
            self.mutation.insert(location, Box::new(element));
            content(self);
            self.mutation.end_new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{elements, tree::Entry};

    #[test]
    fn single_element() {
        let mut tree = Tree::default();
        let mut ui = Ui::new(&mut tree);

        elements::button(&mut ui, "test");

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());
    }
}
