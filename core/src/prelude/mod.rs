
pub mod rust_2021 {
    pub use crate::marker::{Copy, Sized, }; // Send, Sync, Unpin};
    pub use crate::clone::Clone;

    pub use crate::arch::asm;

    pub use crate::result::Result::{self, Err, Ok};
    pub use crate::option::Option::{self, Some, None};

    pub use crate::cmp::{Eq, Ord, PartialEq, PartialOrd};
    pub use crate::convert::{From, Into};

    pub use crate::matches;
}
