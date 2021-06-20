
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Handled(pub bool);

impl Handled {
    pub fn handled(self) -> bool {
        self.0
    }
}

impl From<Handled> for bool {
    fn from(h: Handled) -> Self {
        h.0
    }
}

impl From<bool> for Handled {
    fn from(b: bool) -> Self {
        Handled(b)
    }
}

impl PartialEq<bool> for Handled {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}
