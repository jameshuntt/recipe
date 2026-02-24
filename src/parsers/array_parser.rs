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
//             let start = ingredient.offset.0 + i * child_size;
//             let end = start + child_size;
// 
//             if end > data.len() {
//                 return Err(RecipeError::OutOfBounds);
//             }
// 
//             let element_data = &data[start..end];
    // Use checked math to prevent overflow crashes on malformed input
    let start = ingredient.offset.0
        .checked_add(i.checked_mul(child_size).ok_or(RecipeError::OutOfBounds)?)
        .ok_or(RecipeError::OutOfBounds)?;
    
    let end = start.checked_add(child_size).ok_or(RecipeError::OutOfBounds)?;

    // Standard Bounds Check
    let element_data = data.get(start..end).ok_or(RecipeError::OutOfBounds)?;

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

    #[test]
    fn test_array_parser_multi_field_struct() {
        let parser = ArrayParser::default();

        // Define a struct: { id: u8 (off 0), value: u16 (off 1) } 
        // Total size: 3 bytes (if packed)
        let child_id = Ingredient {
            name: "id".to_string(),
            format: "u8".to_string(),
            offset: Offset(0),
            length: Length(1),
            ..Default::default()
        };

        let child_val = Ingredient {
            name: "value".to_string(),
            format: "u16".to_string(), // Assuming Big Endian parser exists
            offset: Offset(1),
            length: Length(2),
            endianness: crate::config::Endianness::Big,
            ..Default::default()
        };

        let ingredient = Ingredient {
            offset: Offset(0),
            length: Length(6), // 2 elements * 3 bytes
            child_size: Some(3),
            num_elements: Some(2),
            children: Some(vec![child_id, child_val]),
            ..Default::default()
        };

        // Data: [ID: 1, VAL: 0x03E8 (1000)] , [ID: 2, VAL: 0x07D0 (2000)]
        let input = vec![0x01, 0x03, 0xE8, 0x02, 0x07, 0xD0];
        
        let result = parser.parse(&input, &ingredient).expect("Parsing failed");

        if let IngredientValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);

            // Verify Element 0
            if let IngredientValue::Struct(ref map) = arr[0] {
                assert_eq!(map.get("id").unwrap(), &IngredientValue::U8(1));
                // Note: This assumes your parse_table handles U16 correctly
                // assert_eq!(map.get("value").unwrap(), &IngredientValue::U16(1000));
            } else {
                panic!("Element 0 should be a Struct");
            }

            // Verify Element 1
            if let IngredientValue::Struct(ref map) = arr[1] {
                assert_eq!(map.get("id").unwrap(), &IngredientValue::U8(2));
            } else {
                panic!("Element 1 should be a Struct");
            }
        } else {
            panic!("Expected Array variant");
        }
    }
    
    #[test]
    fn test_array_parser_multi_field_struct_2() {
        let parser = ArrayParser::default();

        let child_id = Ingredient {
            name: "id".to_string(),
            format: "u8".to_string(),
            offset: Offset(0),
            length: Length(1),
            ..Default::default()
        };

        let child_val = Ingredient {
            name: "value".to_string(),
            format: "u16".to_string(), 
            offset: Offset(1),
            length: Length(2),
            endianness: crate::config::Endianness::Big,
            ..Default::default()
        };

        let ingredient = Ingredient {
            offset: Offset(0),
            length: Length(6), // 2 elements * 3 bytes (1 + 2)
            child_size: Some(3),
            num_elements: Some(2),
            children: Some(vec![child_id, child_val]),
            ..Default::default()
        };

        // Data: [ID: 1, VAL: 1000 (0x03E8)], [ID: 2, VAL: 2000 (0x07D0)]
        let input = vec![0x01, 0x03, 0xE8, 0x02, 0x07, 0xD0];
        
        let result = parser.parse(&input, &ingredient).expect("Parsing should succeed now");

        if let IngredientValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);

            // Element 1 check
            if let IngredientValue::Struct(ref map) = arr[0] {
                assert_eq!(map.get("id").unwrap(), &IngredientValue::U8(1));
                assert_eq!(map.get("value").unwrap(), &IngredientValue::U16(1000));
            }

            // Element 2 check
            if let IngredientValue::Struct(ref map) = arr[1] {
                assert_eq!(map.get("id").unwrap(), &IngredientValue::U8(2));
                assert_eq!(map.get("value").unwrap(), &IngredientValue::U16(2000));
            }
        } else {
            panic!("Expected IngredientValue::Array");
        }
    }
}