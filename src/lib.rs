//! A collection of individually selectable testing libraries and tools

#[cfg(feature = "use_parameterized")]
pub use ::parameterized;

/// The default choice for a parameterized_options testing library
#[cfg(feature = "use_yare")]
pub use ::yare;

#[cfg(test)]
mod parameterized_options;
