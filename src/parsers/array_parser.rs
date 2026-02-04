use std::collections::HashMap;

use crate::{
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    parsers::parse_table,
    traits::IngredientParser,
};

/// `ArrayParser` is responsible for parsing arrays of binary data into structured
/// `IngredientValue::Array` results.
///
/// This parser handles:
/// - Fixed-size elements based on `child_size`
/// - A known or derived number of elements (`num_elements` or inferred from `length / child_size`)
/// - Either homogeneous children (1 field repeated per element) or heterogeneous structs
///
/// It supports fast parsing of flat arrays when `children.len() == 1`, or full nested parsing
/// for structured children using `parse_table`.
#[derive(Default)]
pub struct ArrayParser;

impl IngredientParser for ArrayParser {
    /// Parses an array ingredient from the provided binary data.
    ///
    /// # Parameters
    /// - `data`: the full byte slice representing the full input buffer
    /// - `ingredient`: the `Ingredient` definition describing this array
    ///
    /// # Returns
    /// A structured `IngredientValue::Array`, where each element is an `IngredientValue::Struct`
    /// built by parsing children fields
    ///
    /// # Errors
    /// - `RecipeError::InvalidConfig` if required fields like `child_size` or `children` are missing
    /// - `RecipeError::OutOfBounds` if computed offsets exceed the data buffer
    fn parse(
        &self,
        data: &[u8],
        ingredient: &Ingredient,
    ) -> Result<IngredientValue, RecipeError> {
        // Required: child element size
        let child_size = ingredient.child_size.ok_or(RecipeError::InvalidConfig(
            "Missing child_size for array".to_string(),
        ))?;

        // Number of array elements
        let num_elements = ingredient
            .num_elements
            .unwrap_or_else(|| ingredient.length.0 / child_size);

        // Required: child field definitions
        let children = ingredient.children.as_ref().ok_or(RecipeError::InvalidConfig(
            "Missing child ingredients for array".to_string(),
        ))?;

        // Fast path: one child, treat each element as flat struct with 1 field
        let parse_fn = if children.len() == 1 {
            let only = &children[0];
            Some(move |data: &[u8]| {
                let val = only.parse(data)?;
                let mut map = HashMap::new();
                map.insert(only.name.clone(), val);
                Ok(IngredientValue::Struct(map))
            })
        } else {
            None
        };

        let mut array_results = Vec::with_capacity(num_elements);

        for i in 0..num_elements {
            let start = ingredient.offset.0 + i * child_size;
            let end = start + child_size;

            if end > data.len() {
                return Err(RecipeError::OutOfBounds);
            }

            let element_data = &data[start..end];

            let element_value = if let Some(ref parse_fn) = parse_fn {
                parse_fn(element_data)?
            } else {
                let mut element_results = HashMap::new();
                for child in children {
                    parse_table(element_data, child, "", &mut element_results)?;
                }
                IngredientValue::Struct(element_results)
            };

            array_results.push(element_value);
        }

        Ok(IngredientValue::Array(array_results))
    }
}

#[cfg(feature = "unsigned")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parser_registry::ParserRegistry,
        types::{Length, Offset}
    };

    #[test]
    fn test_array_parser_single_field() {
        let parser_reg = ParserRegistry::new();
        let parser = parser_reg.get("array").unwrap();

        let child = Ingredient {
            name: "field".to_string(),
            format: "u8".to_string(),
            offset: Offset(0),
            length: Length(1),
            ..Default::default()
        };

        let ingredient = Ingredient {
            offset: Offset(0),
            length: Length(4),
            child_size: Some(1),
            num_elements: Some(4),
            children: Some(vec![child]),
            ..Default::default()
        };

        let input = vec![10, 20, 30, 40];
        let parsed = parser.parse(&input, &ingredient).unwrap();

        match parsed {
            IngredientValue::Array(arr) => {
                assert_eq!(arr.len(), 4);
                for (i, val) in arr.iter().enumerate() {
                    match val {
                            IngredientValue::Struct(map) => {
                            let field = map.get("field").expect("missing field");
                            match field {
                                IngredientValue::U8(v) => assert_eq!(*v, input[i]),
                                _ => panic!("Expected U8 value"),
                            }
                        }
                        _ => panic!("Unexpected element value"),
                    }
                }
            }
            _ => panic!("Expected IngredientValue::Array"),
        }
    }
}