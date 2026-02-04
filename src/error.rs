#[derive(Debug)]
/// The central error domain for the RECIPE engine.
///
/// `RecipeError` provides a deterministic classification of failures occurring 
/// during the lifecycle of a Lens operation—from Blueprint validation to 
/// physical bitstream extraction.
pub enum RecipeError {
    /// Indicates a spatial violation where a Lens attempted to access memory 
    /// outside the allocated physical buffer.
    /// 
    /// ### Security Significance:
    /// This is the primary guard against Buffer Overflow and Out-of-Bounds 
    /// Read vulnerabilities.
    OutOfBounds,

    /// Triggered when the `ParserRegistry` encounters a schema format identifier 
    /// that has not been registered in the current situational context.
    UnsupportedFormat(String),

    /// Indicates that the raw bitstream could not be coerced into the 
    /// target type (e.g., malformed UTF-8 in a string field or invalid 
    /// numeric representation).
    InvalidData(String),

    /// Occurs during recursive parsing (like in `Struct` or `Array` ingredients) 
    /// when a mandatory jurisdictional field is missing from the data stream.
    MissingField(String),

    /// Represents a logical failure in the Blueprint itself. 
    /// 
    /// Used when the XML/JSON configuration defines a structure that is 
    /// mathematically impossible or self-contradictory.
    InvalidConfig(String),

    /// A specific guard for the `Length` primitive.
    ///
    /// Triggered when a segment is defined with a length that violates 
    /// the protocol's minimum requirements (e.g., a 0-byte IP header).
    InvalidLength
}

impl std::fmt::Display for RecipeError {
    /// Transforms internal technical failures into human-readable diagnostic reports.
    ///
    /// This implementation ensures that even in complex, nested parsing scenarios,
    /// the system provides clear attribution for the failure point.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RecipeError::OutOfBounds => write!(f, "Index out of bounds - Spatial violation detected"),
            RecipeError::UnsupportedFormat(fmt) => write!(f, "Unsupported format encountered in registry: {}", fmt),
            RecipeError::InvalidData(msg) => write!(f, "Bitstream coercion failure: {}", msg),
            RecipeError::MissingField(field) => write!(f, "Jurisdictional field missing: {}", field),
            RecipeError::InvalidConfig(msg) => write!(f, "Blueprint configuration error: {}", msg),
            RecipeError::InvalidLength => write!(f, "Spatial constraint violation: Invalid Length"),
        }
    }
}

/// Integration with the standard library Error trait for compatibility 
/// with the broader Rust ecosystem and error-handling crates like `anyhow` or `thiserror`.
impl std::error::Error for RecipeError {}