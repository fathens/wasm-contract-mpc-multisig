#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod error;
pub mod gg20;
pub mod utilities;

pub use error::Error;
pub use error::ErrorType;

pub type Result<A> = core::result::Result<A, error::Error>;
