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


#[cfg(feature = "unsigned")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Length, Offset};

    #[test]
    fn test_struct_parser_recursive_nesting() {
        let parser = StructParser::default();

        // 1. Define Child Ingredients
        let child1 = Ingredient {
            name: "id".to_string(),
            format: "u8".to_string(),
            offset: Offset(0),
            length: Length(1),
            ..Default::default()
        };

        let child2 = Ingredient {
            name: "val".to_string(),
            format: "u8".to_string(),
            offset: Offset(1),
            length: Length(1),
            ..Default::default()
        };

        // 2. Define Parent Struct
        let parent = Ingredient {
            name: "header".to_string(),
            format: "struct".to_string(),
            children: Some(vec![child1, child2]),
            ..Default::default()
        };

        let data = vec![0xAA, 0xBB];
        
        // 3. Execute standard recursive parse
        let result = parser.parse(&data, &parent).expect("Parse failed");

        if let IngredientValue::Struct(map) = result {
            assert_eq!(map.len(), 2);
            assert_eq!(map.get("id").unwrap(), &IngredientValue::U8(0xAA));
            assert_eq!(map.get("val").unwrap(), &IngredientValue::U8(0xBB));
        } else {
            panic!("Expected IngredientValue::Struct");
        }
    }

    #[test]
    fn test_parse_struct_static_with_namespacing() {
        // This tests the static method that uses parse_table
        let child = Ingredient {
            name: "inner_field".to_string(),
            format: "u8".to_string(),
            offset: Offset(0),
            length: Length(1),
            ..Default::default()
        };

        let parent = Ingredient {
            name: "parent_node".to_string(),
            format: "struct".to_string(),
            children: Some(vec![child]),
            ..Default::default()
        };

        let data = vec![0x42];

        // Execute static parse_struct which uses dot-notation
        let result = StructParser::parse_struct(&data, &parent).expect("Static parse failed");

        if let IngredientValue::Struct(map) = result {
            // Because parse_struct calls parse_table(..., &ingredient.name, ...),
            // the key should be "parent_node.inner_field"
            let expected_key = "parent_node.inner_field";
            assert!(map.contains_key(expected_key), "Missing namespaced key: {}", expected_key);
            assert_eq!(map.get(expected_key).unwrap(), &IngredientValue::U8(0x42));
        } else {
            panic!("Expected IngredientValue::Struct");
        }
    }

    #[test]
    fn test_struct_missing_children_error() {
        let parser = StructParser::default();
        let invalid_struct = Ingredient {
            name: "broken".to_string(),
            format: "struct".to_string(),
            children: None, // Missing instructions
            ..Default::default()
        };

        let result = parser.parse(&[0x00], &invalid_struct);
        assert!(result.is_err());
        match result.unwrap_err() {
            RecipeError::InvalidConfig(msg) => assert!(msg.contains("without children")),
            _ => panic!("Expected InvalidConfig error"),
        }
    }
}