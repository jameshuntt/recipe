use crate::{
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    parsers::calculate_bounds,
    traits::{CalculateBounds, IngredientParser}
};

/// The Opaque Payload Extractor.
///
/// `BinaryParser` is responsible for extracting raw, untransformed byte segments
/// from the bitstream. It is commonly used for checksums, cryptographic keys,
/// or forwarding sub-packets to external handlers.
#[derive(Default)]
pub struct BinaryParser;

// Inherits the default boundary calculation logic.
impl CalculateBounds for BinaryParser {}

impl IngredientParser for BinaryParser {
    /// Extracts a physical slice and transforms it into a heap-allocated `Vec<u8>`.
    ///
    /// ### Boundary Enforcement:
    /// This parser relies on `calculate_bounds` to ensure that the extraction
    /// respects the `max_length` safety ceiling defined in the Ingredient.
    fn parse(&self, data: &[u8], ingredient: &Ingredient) -> Result<IngredientValue, RecipeError> {
        // Step 1: Resolve physical memory coordinates.
        // This handles clamping the length and checking for buffer overflows.
        let (start, end) = calculate_bounds(data, ingredient)?;

        // Step 2: Extract and clone the data.
        // Returns a Binary variant containing a owned copy of the memory segment.
        Ok(IngredientValue::Binary(data[start..end].to_vec()))
    }
}