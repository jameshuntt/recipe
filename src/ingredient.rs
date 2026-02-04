use serde::Deserialize;

use crate::{
    config::Endianness,
    error::RecipeError,
    ingredient_value::IngredientValue,
    parser_registry::ParserRegistry,
    types::{Length, Offset}
};

/// The fundamental Descriptor for a data segment.
///
/// An `Ingredient` acts as a set of instructions for the `ParserRegistry`. 
/// It maps a raw, anonymous byte-range to a named, typed, and validated 
/// logical value.
///
/// ### Situational Awareness logic:
/// This structure is highly flexible, supporting both primitive scalars (integers, strings)
/// and complex, nested recursive structures (arrays, structs) through its `children` 
/// and `format` properties.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Ingredient {
    /// The semantic identifier for this field. Used as the key in the final `PacketResult`.
    pub name: String,
    
    /// The physical starting point of this ingredient relative to its parent container.
    pub offset: Offset,
    
    /// The expected spatial extent of the data in bytes.
    pub length: Length,
    
    /// The lookup key used to select the appropriate `IngredientParser` from the Registry.
    /// Example: "u32", "ipv4", "custom_protocol_header".
    pub format: String,

    /// A defensive boundary guard. If the data stream suggests a length 
    /// greater than this, the engine will trigger a jurisdictional violation.
    pub max_length: Option<Length>,

    /// RECURSIVE LENSING:
    /// Enables the nesting of ingredients. If this is present, the parser 
    /// will treat the current segment as a container (Struct or Array) 
    /// and apply these child-lenses to the inner data.
    pub children: Option<Vec<Ingredient>>,

    /// The fixed footprint of a single repeating element (used for optimized Array parsing).
    pub child_size: Option<usize>,

    /// The iteration count for repeating segments.
    pub num_elements: Option<usize>,

    /// A fallback value to be used if the lensing operation fails but 
    /// the protocol allows for non-critical "missing" data.
    pub default: Option<IngredientValue>,

    /// The bit-order policy applied during numeric reconstruction.
    pub endianness: Endianness,
}

impl Ingredient {
    /// Executes the lensing operation on a raw byte buffer.
    ///
    /// This is a high-level convenience method that instantiates a local 
    /// `ParserRegistry` to resolve the format and extract the value.
    ///
    /// ### Technical Performance Note:
    /// For high-throughput scenarios (500k+ packets), it is recommended to 
    /// pass a pre-allocated `ParserRegistry` instead of creating one per call.
    pub fn parse(&self, data: &[u8]) -> Result<IngredientValue, RecipeError> {
        ParserRegistry::new().parse_ingredient(data, self)
    }
    
    /// Verifies that the requested `format` exists within the current "Registry of Knowledge."
    ///
    /// This acts as a pre-flight check during the "Blueprint Loading" phase,
    /// ensuring that the engine won't encounter an `UnsupportedFormat` error 
    /// in the middle of a critical real-time parsing loop.
    pub fn validate_format(&self, registry: &ParserRegistry) -> Result<(), RecipeError> {
        if !registry.contains(&self.format) {
            return Err(RecipeError::UnsupportedFormat(self.format.clone()));
        }
        Ok(())
    }
}