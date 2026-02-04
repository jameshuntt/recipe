use serde::Deserialize;
use crate::error::RecipeError;

/// A domain-specific primitive representing the spatial extent of a data segment.
///
/// `Length` acts as a semantic guard, preventing the accidental mixing of raw 
/// integers with memory-sensitive offset and size calculations. 
///
/// ### Architectural Role
/// In the `RECIPE` ecosystem, this structure defines the 'Viewport' size of a Lens.
/// By using `#[repr(transparent)]`, we ensure that this safety wrapper has zero 
/// runtime overhead compared to a raw `usize`, while providing compile-time 
/// boundary enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[repr(transparent)]
pub struct Length(pub usize);

impl Length {
    /// Returns `true` if the length represents a null/empty segment.
    ///
    /// Useful for early-exit logic in parsers where zero-length ingredients 
    /// do not require a memory slice operation.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Validates the spatial integrity of the segment.
    ///
    /// ### Error Handling
    /// Returns `RecipeError::InvalidLength` if the segment is empty.
    /// This is a critical check for parsers that expect non-null payloads (e.g., Numeric, IP).
    pub fn validate(&self) -> Result<(), RecipeError> {
        if self.0 == 0 {
            Err(RecipeError::InvalidLength)
        } else {
            Ok(())
        }
    }

    /// Computes a new length including a padding overhead.
    ///
    /// This is used during the "Recipe Planning" stage to account for alignment,
    /// footers, or bit-stuffing requirements in specialized protocols.
    pub fn padded(&self, pad: usize) -> Self {
        Length(self.0 + pad)
    }
}

// -----------------------------------------------------------------------------
// MACRO EXPANSION: Default Value Injection
// -----------------------------------------------------------------------------
// Standardizes the initialization of Length across the crate.
// Defaulting to 1 ensures that new ingredients are non-zero by default,
// aligning with the "Safe by Default" philosophy of the framework.
impl_newtype_default!(Length, 1);