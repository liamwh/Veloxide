//! Key default types for this application designed to be imported in most crate modules.
//!
//! Notes:
//!     - The best practice is to have a narrow crate prelude to normalize the key types throughout the application code.
//!     - We keep this as small as possible, and try to limit generic name beside Result and Error (which is re-exported from this module)

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern, mostly for external type to type From/TryFrom conversions
// See this video as a reference: https://www.youtube.com/watch?v=oxx7MmN4Ib0
pub struct W<T>(pub T);
