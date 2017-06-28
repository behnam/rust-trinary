use std::convert::From;
use std::default::Default;

/// A three-valued type equivalent to `Option<bool>`.
///
/// It also provides several convenience methods for testing tri-state values.
#[derive(Debug, Clone, Copy)]
pub enum Kleene {
    /// The tri-state value signifying definitive truth.
    True,

    /// The tri-state value signifying definitive falsity.
    False,

    /// The tri-state value signifying an unknown truth value.
    Unknown,
}

impl Kleene {
    /// Is `self` equal to `True`?
    pub fn is_true(self) -> bool {
        match self {
            Kleene::True => true,
            _ => false,
        }
    }

    /// Is `self` equal to `False`?
    pub fn is_false(self) -> bool {
        match self {
            Kleene::False => true,
            _ => false,
        }
    }

    /// Is `self` equal to `Unknown`?
    pub fn is_unknown(self) -> bool {
        match self {
            Kleene::Unknown => true,
            _ => false,
        }
    }

    /// Converts `self` to an `Option<bool>`. Equivalent to `Option::<bool>::from(self)`.
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Kleene::True => Some(true),
            Kleene::False => Some(false),
            Kleene::Unknown => None,
        }
    }
}

impl From<Kleene> for Option<bool> {
    fn from(t: Kleene) -> Option<bool> {
        t.to_bool()
    }
}

impl From<Option<bool>> for Kleene {
    fn from(b: Option<bool>) -> Kleene {
        match b {
            Some(true) => Kleene::True,
            Some(false) => Kleene::False,
            None => Kleene::Unknown,
        }
    }
}

impl From<bool> for Kleene {
    fn from(b: bool) -> Kleene {
        match b {
            true => Kleene::True,
            false => Kleene::False,
        }
    }
}

impl Default for Kleene {
    fn default() -> Self {
        Kleene::Unknown
    }
}
