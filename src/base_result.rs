//! Result type and conversion utilities.
//!
//! This module provides a common result type used throughout the library
//! and a trait for converting various result types to this common format.

/// A type alias for Result with a boxed error trait object.
///
/// This is used throughout the library as a common return type for fallible operations.
pub type BaseResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Trait for converting various Result types to BaseResult.
///
/// This trait allows automatic conversion of different error types into
/// the library's standard BaseResult type.
pub trait ToBaseResult<T> {
    /// Converts self into a BaseResult.
    fn to_base_result(self) -> Result<T, Box<dyn std::error::Error>>;
}

impl<T, E> ToBaseResult<T> for std::result::Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_base_result(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(format!("{:?}", error).into()),
        }
    }
}

// pub trait Clone<T> {
//     fn clone(&self) -> BaseResult<T>;
// }

// impl<T> Clone<T> for Result<T, Box<dyn std::error::Error>>
// where
//     T: std::clone::Clone,
// {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Ok(arg0) => Self::Ok(arg0.clone()),
//             Self::Err(error) => Err(format!("{:?}", error).into()),
//         }
//     }
// }
