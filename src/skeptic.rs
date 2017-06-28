use std::convert::From;
use std::default::Default;

/// A three-valued type equivalent to `Option<bool>`.
///
/// It also provides several convenience methods for testing tri-state values.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd)]
pub enum Skeptic {
    /// The tri-state value signifying definitive truth.
    True,

    /// The tri-state value signifying definitive falsity.
    False,

    /// The tri-state value signifying an unknown truth value.
    Unknown,
}

impl Skeptic {
    /// Is `self` equal to `True`?
    pub fn is_true(self) -> bool {
        self == Skeptic::True
    }

    /// Is `self` equal to `False`?
    pub fn is_false(self) -> bool {
        self == Skeptic::False
    }

    /// Is `self` equal to `Unknown`?
    pub fn is_unknown(self) -> bool {
        self == Skeptic::Unknown
    }

    /// Converts `self` to an `Option<bool>`. Equivalent to `Option::<bool>::from(self)`.
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Skeptic::True => Some(true),
            Skeptic::False => Some(false),
            Skeptic::Unknown => None,
        }
    }
}

impl PartialEq for Skeptic {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Skeptic::True => other.is_true(),
            Skeptic::False => other.is_false(),
            Skeptic::Unknown => false,
        }
    }
}

impl From<Skeptic> for Option<bool> {
    fn from(t: Skeptic) -> Option<bool> {
        t.to_bool()
    }
}

impl From<Option<bool>> for Skeptic {
    fn from(b: Option<bool>) -> Skeptic {
        match b {
            Some(true) => Skeptic::True,
            Some(false) => Skeptic::False,
            None => Skeptic::Unknown,
        }
    }
}

impl From<bool> for Skeptic {
    fn from(b: bool) -> Skeptic {
        match b {
            true => Skeptic::True,
            false => Skeptic::False,
        }
    }
}

impl Default for Skeptic {
    fn default() -> Self {
        Skeptic::Unknown
    }
}
