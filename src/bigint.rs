/// Provides a common interface for bigints, so bigint backends can be changed.
/// Right now we are just using the types in a generic enough manner that
/// simply re-exporting the appropriate type is enough

#[cfg(feature = "rug")]
pub use rug::Integer as BigInt;

#[cfg(feature = "num-bigint")]
pub use num_bigint::BigInt;

#[cfg(feature = "i128")]
pub use i128 as BigInt;
