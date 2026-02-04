use crate::{
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    parsers::calculate_bounds,
    traits::IngredientParser
};

/// The Textual Data Interpreter.
///
/// `StringParser` extracts segments of memory and interprets them as UTF-8 text.
/// It is designed for robustness in "noisy" environments where data may 
/// contain non-standard characters.
#[derive(Default)]
pub struct StringParser;

impl IngredientParser for StringParser {
    /// Decodes a byte slice into a valid UTF-8 String.
    ///
    /// ### Resilience Strategy:
    /// This parser uses "Lossy" decoding. Instead of failing the entire 
    /// parsing operation if an invalid UTF-8 sequence is encountered, it 
    /// replaces the corruption with the replacement character (). This 
    /// ensures that logging and situational awareness continue even in 
    /// degraded signal conditions.
    fn parse(&self, data: &[u8], ingredient: &Ingredient) -> Result<IngredientValue, RecipeError> {
        // Step 1: Secure the boundaries.
        // Uses the centralized clamping logic to prevent buffer overflows.
        let (start, end) = calculate_bounds(data, ingredient)?;
        
        // Step 2: Lossy UTF-8 Interpretation.
        // We slice the data and convert it. Using lossy conversion prevents
        // a single bit-flip from crashing the entire protocol ingestion.
        let string_value = String::from_utf8_lossy(&data[start..end]).to_string();
        
        // Step 3: Wrap in the universal Value type.
        Ok(IngredientValue::String(string_value))
    }
}