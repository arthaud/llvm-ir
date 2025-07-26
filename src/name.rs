use std::fmt;
use std::sync::Arc;

/// Many LLVM objects have a `Name`, which is either a string name, or just a
/// sequential numbering (e.g. `%3`).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
pub enum Name {
    /// has a string name
    // with `Arc`, the enum `Name` has size 16 bytes, vs with a `String`
    // directly, the enum `Name` has size 32 bytes. This has implications also
    // for the size of other important structures, such as `Operand`.
    // Using `Arc<String>` allows sharing identical names across the module,
    // reducing memory usage when the same register names are referenced
    // multiple times.
    Name(Arc<String>),
    /// doesn't have a string name and was given this sequential number
    Number(usize),
}

impl Name {
    pub(crate) fn name_or_num(s: String, ctr: &mut usize, interner: &mut crate::from_llvm::StringInterner) -> Self {
        if s.is_empty() {
            let rval = Name::Number(*ctr);
            *ctr += 1;
            rval
        } else {
            Name::Name(interner.intern(s))
        }
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Name::Name(Arc::new(s))
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name::Name(Arc::new(s.into()))
    }
}

impl From<usize> for Name {
    fn from(u: usize) -> Self {
        Name::Number(u)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Name::Name(s) => write!(f, "%{}", s),
            Name::Number(n) => write!(f, "%{}", n),
        }
    }
}
