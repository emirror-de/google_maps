use serde::{Serialize, Deserialize};

/// A representation of duration as a numeric value and a display string.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Duration {
    /// A string representation of the duration value.
    text: String,
    /// The duration in seconds.
    value: u32,
} // struct