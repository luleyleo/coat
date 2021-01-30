use std::{any::Any, marker::PhantomData};

pub(crate) type SelectorSymbol = &'static str;

#[derive(Debug, PartialEq, Eq)]
pub struct Selector<T = ()>(SelectorSymbol, PhantomData<T>);

#[derive(Debug)]
pub struct Action {
    symbol: SelectorSymbol,
    payload: Box<dyn Any>,
}

impl Selector<()> {
    /// A selector that does nothing.
    pub const NOOP: Selector = Selector::new("");
}

impl<T> Selector<T> {
    /// Create a new `Selector` with the given string.
    pub const fn new(s: &'static str) -> Selector<T> {
        Selector(s, PhantomData)
    }

    /// Returns the `SelectorSymbol` identifying this `Selector`.
    pub(crate) const fn symbol(self) -> SelectorSymbol {
        self.0
    }
}

impl<T: Any> Selector<T> {
    pub fn with(self, payload: T) -> Action {
        Action::new(self, payload)
    }
}

impl Action {
    pub fn new<T: Any>(selector: Selector<T>, payload: T) -> Self {
        Action {
            symbol: selector.symbol(),
            payload: Box::new(payload),
        }
    }

    /// Returns `true` if `self` matches this `selector`.
    pub fn is<T>(&self, selector: Selector<T>) -> bool {
        self.symbol == selector.symbol()
    }

    /// Returns `Some(&T)` (this `Command`'s payload) if the selector matches.
    ///
    /// Returns `None` when `self.is(selector) == false`.
    ///
    /// Alternatively you can check the selector with [`is`] and then use [`get_unchecked`].
    ///
    /// # Panics
    ///
    /// Panics when the payload has a different type, than what the selector is supposed to carry.
    /// This can happen when two selectors with different types but the same key are used.
    ///
    /// [`is`]: #method.is
    /// [`get_unchecked`]: #method.get_unchecked
    pub fn get<T: Any>(&self, selector: Selector<T>) -> Option<&T> {
        if self.symbol == selector.symbol() {
            Some(self.payload.downcast_ref().unwrap_or_else(|| {
                panic!(
                    "The selector \"{}\" exists twice with different types. See druid::Command::get for more information",
                    selector.symbol()
                );
            }))
        } else {
            None
        }
    }

    /// Returns a reference to this `Command`'s payload.
    ///
    /// If the selector has already been checked with [`is`], then `get_unchecked` can be used safely.
    /// Otherwise you should use [`get`] instead.
    ///
    /// # Panics
    ///
    /// Panics when `self.is(selector) == false`.
    ///
    /// Panics when the payload has a different type, than what the selector is supposed to carry.
    /// This can happen when two selectors with different types but the same key are used.
    ///
    /// [`is`]: #method.is
    /// [`get`]: #method.get
    pub fn get_unchecked<T: Any>(&self, selector: Selector<T>) -> &T {
        self.get(selector).unwrap_or_else(|| {
            panic!(
                "Expected selector \"{}\" but the command was \"{}\".",
                selector.symbol(),
                self.symbol
            )
        })
    }
}
