use crate::{
    config::Endianness,
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    types::{Length, Offset, RecipeResult}
};

/// A fluent constructor for the `Ingredient` domain model.
///
/// `IngredientBuilder` implements the Builder Pattern to provide a safe,
/// incremental way to define jurisdictional boundaries. It ensures that 
/// situational awareness properties (like endianness and max length) are 
/// correctly initialized before the "Recipe" is finalized.
pub struct IngredientBuilder {
    name: String,
    offset: Offset,
    length: Length,
    format: String,
    max_length: Option<Length>,
    children: Option<Vec<Ingredient>>,
    child_size: Option<usize>,
    num_elements: Option<usize>,
    default: Option<IngredientValue>,
    endianness: Endianness
}

impl IngredientBuilder {
    /// Convenience method for setting the start boundary using raw primitives.
    ///
    /// Transforms a `usize` into a type-safe `Offset`, maintaining the 
    /// spatial jurisdiction of the resulting Lens.
    pub fn offset_bytes(self, offset: usize) -> Self {
        self.offset(Offset(offset))
    }

    /// Convenience method for setting the spatial extent using raw primitives.
    ///
    /// Ensures the builder remains ergonomic while enforcing the internal 
    /// use of the `Length` safety wrapper.
    pub fn length_bytes(self, length: usize) -> Self {
        self.length(Length(length))
    }
}

impl IngredientBuilder {
    /// Initializes a blank-slate builder with safe defaults.
    ///
    /// By default, the builder assumes a zero-offset, zero-length 
    /// configuration with Little Endian bit-ordering.
    pub fn new() -> Self {
        IngredientBuilder {
            name: String::new(),
            offset: Offset(0),
            length: Length(0),
            format: String::new(),
            max_length: None,
            children: None,
            child_size: None,
            num_elements: None,
            default: None,
            endianness: Endianness::Little
        }
    }

    /// Assigns a semantic identifier to the ingredient.
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Sets the precise starting address for this lens.
    pub fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the expected byte-width of the data segment.
    pub fn length(mut self, length: Length) -> Self {
        self.length = length;
        self
    }

    /// Defines the format identifier for Parser Registry lookup (e.g., "u64", "ipv4").
    pub fn format(mut self, format: &str) -> Self {
        self.format = format.to_string();
        self
    }

    /// Imposes a hard safety ceiling on the ingredient's length.
    ///
    /// Used to prevent "Greedy Parsing" vulnerabilities in variable-length fields.
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(Length(max_length));
        self
    }

    /// RECURSIVE ASSEMBLY:
    /// Injects a sub-recipe of child ingredients for composite types (Structs/Arrays).
    pub fn children(mut self, children: Vec<Ingredient>) -> Self {
        self.children = Some(children);
        self
    }

    /// Specifies the memory footprint of a single element in a repeating array.
    pub fn child_size(mut self, child_size: usize) -> Self {
        self.child_size = Some(child_size);
        self
    }

    /// Defines the iteration count for the lens.
    pub fn num_elements(mut self, num_elements: usize) -> Self {
        self.num_elements = Some(num_elements);
        self
    }

    /// Provides a fallback value to ensure system resilience during parsing failures.
    pub fn default(mut self, default: IngredientValue) -> Self {
        self.default = Some(default);
        self
    }

    /// Configures the lens to interpret numeric data as Little Endian (Standard x86).
    pub fn little_endian(mut self) -> Self {
        self.endianness = Endianness::Little;
        self
    }

    /// Configures the lens for Big Endian bit-ordering (Network Byte Order).
    pub fn big_endian(mut self) -> Self {
        self.endianness = Endianness::Big;
        self
    }

    /// Finalizes the construction, consuming the builder and returning 
    /// a static `Ingredient` lens.
    pub fn build(self) -> Ingredient {
        Ingredient {
            name: self.name,
            offset: self.offset,
            length: self.length,
            format: self.format,
            max_length: self.max_length,
            children: self.children,
            child_size: self.child_size,
            num_elements: self.num_elements,
            default: self.default,
            endianness: self.endianness
        }
    }

    /// Performs a pre-flight structural integrity check.
    ///
    /// ### Validation Logic:
    /// - **Identity Check**: Ensures the ingredient has a name for result mapping.
    /// - **Spatial Check**: Prevents the creation of zero-length lenses, which 
    ///   could lead to infinite loops or logic errors in parsers.
    pub fn validate(&self) -> RecipeResult<()> {
        if self.name.is_empty() {
            return Err(RecipeError::InvalidConfig(
                "Ingredient name cannot be empty".to_string()
            ));
        }

        if self.length.0 == 0 {
            return Err(RecipeError::InvalidConfig(
                "Ingredient length must be greater than zero".to_string()
            ));
        }

        Ok(())
    }
}