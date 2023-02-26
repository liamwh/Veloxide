pub mod memory;
pub mod postgres;
pub use crate::domain::Todo;

// Re-exports
pub use crate::prelude::*;
pub use memory::*;
pub use postgres::*;

// NOTE: Enum dispatch is a much more performant implementation of dynamic dispatch if this code was going to go to production.
// https://docs.rs/enum_dispatch/latest/enum_dispatch/
