use std::collections::HashMap;

use crate::{
    error::RecipeError, 
    ingredient::Ingredient, 
    ingredient_value::IngredientValue, 
    parsers::parse_table, 
    traits::IngredientParser
};

/// The Hierarchical Object Engine.
///
/// `StructParser` enables the recursive nesting capabilities of the RECIPE engine.
/// It treats a segment of data as a "Parent Jurisdiction," delegating the 
/// actual bit-parsing to its internal "Children."
#[derive(Default)]
pub struct StructParser;

impl IngredientParser for StructParser {
    /// Performs a recursive extraction of a data structure.
    ///
    /// ### Logical Flow:
    /// 1. **Blueprint Verification**: Checks if the Ingredient actually defines child fields.
    /// 2. **Delegated Execution**: Iterates through each child, triggering their 
    ///    specific parsers (Scalars, Strings, or even other Structs).
    /// 3. **Aggregation**: Collects results into a local `HashMap` before returning
    ///    the final `IngredientValue::Struct`.
    fn parse(
        &self,
        data: &[u8],
        ingredient: &Ingredient
    ) -> Result<IngredientValue, RecipeError> {
        let mut results = HashMap::new();
        
        // Ensure this struct isn't a "leaf node" by mistake.
        if let Some(children) = &ingredient.children {
            for child in children {
                // RECURSION POINT: The child resolves its own value 
                // based on its internal Blueprint.
                let value = child.parse(data)?;
                results.insert(child.name.clone(), value);
            }
            Ok(IngredientValue::Struct(results))
        } else {
            // Safety: A struct without instructions is a configuration breach.
            Err(RecipeError::InvalidConfig(
                "Struct without children".to_string()
            ))
        }
    }
}

impl StructParser {
    /// A specialized static entry point for table-driven struct parsing.
    ///
    /// Unlike the standard `parse` method, this version integrates with 
    /// `parse_table` to handle dot-notated namespacing (e.g., "Parent.Child") 
    /// and global result accumulation.
    pub fn parse_struct(data: &[u8], ingredient: &Ingredient) -> Result<IngredientValue, RecipeError> {
        let mut struct_results = HashMap::new();
        
        // JURISDICTION CHECK: Verify children exist in the manifest.
        let child_ingredients = ingredient.children.as_ref()
            .ok_or(RecipeError::InvalidConfig("No child ingredients defined for struct".to_string()))?;

        for child in child_ingredients {
            // Use the global table parser to resolve the child within the 
            // context of the parent's name.
            parse_table(data, child, &ingredient.name, &mut struct_results)?;
        }

        Ok(IngredientValue::Struct(struct_results))
    }
}