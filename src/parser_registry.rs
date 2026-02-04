use crate::{
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    parsers::*,
    types::{ParserTrait, RecipeResult}
};

use std::collections::HashMap;

/// The central Orchestrator for Ingredient Lensing.
///
/// `ParserRegistry` maintains a map of all available parsing capabilities.
/// It acts as a "Service Locator" for the RECIPE engine, decoupling the 
/// definition of an Ingredient from the actual logic used to decode it.
pub struct ParserRegistry {
    /// A collection of heap-allocated, thread-safe trait objects.
    /// This allows for runtime selection of parsing logic based on 
    /// the "format" string provided in the Blueprint.
    parsers: HashMap<String, Box<ParserTrait>>,
}

impl ParserRegistry {
    /// Constructs a new Registry and populates it with enabled parsing capabilities.
    ///
    /// ### Feature-Gated Initialization:
    /// This constructor uses conditional compilation to only register parsers
    /// that have been explicitly enabled. This ensures that the Registry 
    /// memory footprint is minimized in specialized environments.
    pub fn new() -> Self {
        let mut parsers: HashMap<_, Box<ParserTrait>> = HashMap::new();
        
        // --- Core Protocol Parsers ---
        register_parsers!(parsers,
            "array" => ArrayParser,
            "struct" => StructParser,
            "binary" => BinaryParser,
            "string" => StringParser,
        );

        // --- Signed Numeric Jurisdiction ---
        #[cfg(feature = "signed")]
        register_parsers!(parsers,
            "i8" => I8Parser,
            "i16" => I16Parser,
            "i32" => I32Parser,
            "i64" => I64Parser,
        );

        // --- Unsigned Numeric Jurisdiction ---
        #[cfg(feature = "unsigned")]
        register_parsers!(parsers,
            "u8" => U8Parser,
            "u16" => U16Parser,
            "u32" => U32Parser,
            "u64" => U64Parser,
        );

        // --- Floating Point Capability ---
        #[cfg(feature = "float")]
        register_parsers!(parsers,
            "f32" => F32Parser,
            "f64" => F64Parser,
        );

        // --- Network Layer Awareness ---
        #[cfg(feature = "network")]
        register_parsers!(parsers,
            "ipv4" => IpV4Parser,
            "ipv6" => IpV6Parser,
        );

        Self { parsers }
    }

    /// Primary entry point for data extraction.
    ///
    /// Resolves the requested `format` against the internal registry and 
    /// delegates the byte-slicing logic to the associated `ParserTrait`.
    pub fn parse(
        &self,
        format: &str,
        data: &[u8],
        ingredient: &Ingredient
    ) -> RecipeResult<IngredientValue> {
        if let Some(parser) = self.parsers.get(format) {
            // SITUATIONAL AWARENESS: The parser is invoked with the 
            // full data context and the specific ingredient's rules.
            parser.parse(data, ingredient)
        } else {
            Err(RecipeError::UnsupportedFormat(format.to_string()))
        }
    }

    /// Specialized wrapper for parsing a high-level `Ingredient` structure.
    ///
    /// Validates that the ingredient's requested format exists before 
    /// attempting to execute the lens.
    pub fn parse_ingredient(
        &self,
        data: &[u8],
        ing: &Ingredient
    ) -> RecipeResult<IngredientValue> {
        if self.contains(&ing.format) {
            self.parse(&ing.format, data, ing)
        } else {
            Err(RecipeError::UnsupportedFormat(ing.format.to_string()))
        }
    }

    /// Retrieves a reference to a specific parser trait object.
    ///
    /// Useful for recursive parsers (like Arrays or Structs) that need 
    /// to look up their children's parsers within the same registry.
    pub fn get(&self, format: &str) -> RecipeResult<&ParserTrait> {
        self.parsers
            .get(format)
            .map(|boxed| boxed.as_ref())
            .ok_or_else(|| RecipeError::UnsupportedFormat(format.to_string()))
    }

    /// Checks if a specific "Situational Capability" is registered.
    pub fn contains(&self, format: &str) -> bool {
        self.parsers.contains_key(format)
    }
}

// -----------------------------------------------------------------------------
// UNIT TESTING JURISDICTION
// -----------------------------------------------------------------------------
#[cfg(feature = "unsigned")]
#[cfg(test)]
mod tests {
    use super::ParserRegistry;
    use crate::{
        types::Length,
        ingredient_builder::IngredientBuilder,
        ingredient_value::IngredientValue,
    };

    #[test]
    fn parses_u8_correctly() {
        let registry = ParserRegistry::new();

        // Testing the integration between the Builder and the Registry.
        let ing = IngredientBuilder::new()
            .length(Length(1))
            .build(); 

        let result = registry
            .parse("u8", &[42], &ing);

        // Verification of the deterministic output.
        assert_eq!(result.unwrap(), IngredientValue::U8(42));
    }
}