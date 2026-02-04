use serde::Deserialize;

/// A type-safe representation of a memory-relative displacement (Address Offset).
///
/// ### The "Spatial Jurisdiction" Anchor
/// In a zero-copy "Recipe" system, `Offset` is the most critical primitive for 
/// preventing spatial overlap errors. It defines the starting bit/byte boundary 
/// for a Lens, ensuring that data extraction begins at the exact deterministic 
/// location dictated by the protocol schema.
///
/// By utilizing `#[repr(transparent)]`, we maintain the performance of a raw `usize` 
/// while enforcing a type-boundary that prevents developers from accidentally 
/// using a `Length` as an `Offset` (a common source of "Off-by-One" security vulnerabilities).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[repr(transparent)]
pub struct Offset(pub usize);

impl Offset {
    /// Validates whether the current offset satisfies a specific hardware or 
    /// protocol alignment requirement.
    ///
    /// ### Technical Significance:
    /// Many high-performance CPU instructions (and SIMD operations used in your 
    /// framing logic) require data to be aligned to 4, 8, or 16-byte boundaries. 
    /// This method allows the `RECIPE` engine to pre-verify alignment before 
    /// attempting a zero-copy cast, avoiding bus errors or performance penalties.
    pub fn aligned(&self, align: usize) -> bool {
        // Deterministic check for memory alignment.
        self.0 % align == 0
    }
}

// -----------------------------------------------------------------------------
// DEFAULT JURISDICTION
// -----------------------------------------------------------------------------
// This macro initializes the Offset. 
// Note: Defaulting to 1 (rather than 0) can act as a "Boundary Guard" to ensure 
// that offsets are consciously set by the Blueprint rather than falling back 
// to a potentially unsafe null-offset by accident.
impl_newtype_default!(Offset, 1);