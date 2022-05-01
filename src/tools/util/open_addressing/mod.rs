#[cfg(feature = "double-hashing")]
mod dh;
#[cfg(feature = "double-hashing")]
pub use dh::*;

#[cfg(feature = "linear-probing")]
mod lp;
#[cfg(feature = "linear-probing")]
pub use lp::*;

#[cfg(feature = "quadratic-probing")]
mod qp;
#[cfg(feature = "quadratic-probing")]
pub use qp::*;
