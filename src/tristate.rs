use std::convert::From;
use std::default::Default;

/// A three-valued type equivalent to `Option<bool>`.
///
/// It also provides several convenience methods for testing tri-state values.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TriState {
    /// The tri-state value signifying definitive truth.
    True,

    /// The tri-state value signifying definitive falsity.
    False,

    /// The tri-state value signifying an unknown truth value.
    Unknown,
}

impl TriState {
    /// Is `self` equal to `True`?
    pub fn is_true(self) -> bool {
        self == TriState::True
    }

    /// Is `self` equal to `False`?
    pub fn is_false(self) -> bool {
        self == TriState::False
    }

    /// Is `self` equal to `Unknown`?
    pub fn is_unknown(self) -> bool {
        self == TriState::Unknown
    }

    /// Converts `self` to an `Option<bool>`. Equivalent to `Option::<bool>::from(self)`.
    pub fn to_bool(self) -> Option<bool> {
        match self {
            TriState::True => Some(true),
            TriState::False => Some(false),
            TriState::Unknown => None,
        }
    }
}

impl From<TriState> for Option<bool> {
    fn from(t: TriState) -> Option<bool> {
        t.to_bool()
    }
}

impl From<Option<bool>> for TriState {
    fn from(b: Option<bool>) -> TriState {
        match b {
            Some(true) => TriState::True,
            Some(false) => TriState::False,
            None => TriState::Unknown,
        }
    }
}

impl From<bool> for TriState {
    fn from(b: bool) -> TriState {
        match b {
            true => TriState::True,
            false => TriState::False,
        }
    }
}

impl Default for TriState {
    fn default() -> Self {
        TriState::Unknown
    }
}
