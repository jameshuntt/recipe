use crate::ingredient_value::IngredientValue;
use crate::error::RecipeError;
use std::collections::HashMap;

/// The Domain-Specific Payload.
///
/// This structure represents the "Hydrated" state of your data. While 
/// `IngredientValue` is a generic container, `Payload` is the high-level 
/// representation that your application-level logic actually consumes.
#[derive(Debug)]
pub struct Payload {
    pub magic_number: String,
    pub timestamp: u64,
    pub entries: Vec<EntryPayload>,
}

/// A nested domain unit representing an individual record.
#[derive(Debug)]
pub struct EntryPayload {
    pub id: u32,
    pub value: f32,
    pub label: String,
}

impl Payload {
    /// THE HYDRATION LOGIC:
    /// Maps a flattened collection of extracted ingredients into a 
    /// structured, type-safe Domain Model.
    ///
    /// ### Error Handling:
    /// This method enforces strict structural integrity. It validates not just 
    /// that the fields exist (`MissingField`), but that they are of the 
    /// expected type (`InvalidData`).
    pub fn from_ingredients(ingredients: &HashMap<String, IngredientValue>) -> Result<Self, RecipeError> {
        
        // 1. HEADER EXTRACTION: Extract and clone the magic signature.
        let magic_number = ingredients.get("header.magic_number")
            .ok_or_else(|| RecipeError::MissingField("header.magic_number".to_string()))?
            .as_string()
            .ok_or_else(|| RecipeError::InvalidData("Invalid magic_number".to_string()))?
            .clone();

        // 2. TEMPORAL METADATA: Coerce the body timestamp into a native u64.
        let timestamp = *ingredients.get("body.timestamp")
            .ok_or_else(|| RecipeError::MissingField("body.timestamp".to_string()))?
            .as_u64()
            .ok_or_else(|| RecipeError::InvalidData("Invalid timestamp".to_string()))?;

        // 3. RECURSIVE ARRAY HYDRATION:
        // Iterates through the list of entries, treating each element as a nested 
        // structure (HashMap) to build the `EntryPayload` collection.
        let entries = ingredients.get("body.entries")
            .ok_or_else(|| RecipeError::MissingField("body.entries".to_string()))?
            .as_array()
            .ok_or_else(|| RecipeError::InvalidData("Invalid entries".to_string()))?
            .iter()
            .map(|entry| {                
                // Ensure the array element is indeed a Struct (Map).
                let entry_map = match entry {
                    IngredientValue::Struct(map) => map,
                    _ => return Err(RecipeError::InvalidData("Expected struct in entries array".to_string())),
                };
        
                // Extract individual record fields with strict type coercion.
                let id = *entry_map.get("id")
                    .ok_or_else(|| RecipeError::MissingField("entry.id".to_string()))?
                    .as_u32()
                    .ok_or_else(|| RecipeError::InvalidData("Invalid entry id".to_string()))?;
        
                let value = *entry_map.get("value")
                    .ok_or_else(|| RecipeError::MissingField("entry.value".to_string()))?
                    .as_f32()
                    .ok_or_else(|| RecipeError::InvalidData("Invalid entry value".to_string()))?;
        
                let label = entry_map.get("label")
                    .ok_or_else(|| RecipeError::MissingField("entry.label".to_string()))?
                    .as_string()
                    .ok_or_else(|| RecipeError::InvalidData("Invalid entry label".to_string()))?
                    .clone();
        
                Ok(EntryPayload { id, value, label })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Payload {
            magic_number,
            timestamp,
            entries,
        })
    }
}

#[test]
fn test_payload_from_ingredients() {
    let mut map = HashMap::new();
    map.insert("header.magic_number".to_string(), IngredientValue::String("MAGIC".into()));
    map.insert("body.timestamp".to_string(), IngredientValue::U64(42));
    map.insert("body.entries".to_string(), IngredientValue::Array(vec![
        IngredientValue::Struct(HashMap::from([
            ("id".into(), IngredientValue::U32(1)),
            ("value".into(), IngredientValue::F32(3.14)),
            ("label".into(), IngredientValue::String("first".into())),
        ])),
    ]));

    let payload = Payload::from_ingredients(&map).unwrap();

    assert_eq!(payload.magic_number, "MAGIC");
    assert_eq!(payload.timestamp, 42);
    assert_eq!(payload.entries[0].id, 1);
}

#[cfg(test)]
mod tests {
    use crate::config::Endianness;
    use crate::ingredient::Ingredient;
    use crate::parser_registry::ParserRegistry;
    use crate::types::{Length, Offset};
    use crate::ingredient_value::IngredientValue;

    /// PROOF OF BIT-ORDER AGNOSTICISM:
    /// This test verifies that a U32 encoded in Big Endian and one in Little Endian
    /// both resolve to the same logical value (0x12345678) when the correct 
    /// 'Lens' (Ingredient) is applied.
    #[test]
    fn test_u32_endianness() {
        let big_endian_bytes = 0x12345678u32.to_be_bytes(); // [0x12, 0x34, 0x56, 0x78]
        let little_endian_bytes = 0x12345678u32.to_le_bytes(); // [0x78, 0x56, 0x34, 0x12]

        let ingredient_be = Ingredient {
            name: "u32_be".to_string(),
            offset: Offset(0),
            length: Length(4),
            format: "u32".to_string(),
            endianness: Endianness::Big,
            ..Default::default()
        };

        let ingredient_le = Ingredient {
            name: "u32_le".to_string(),
            offset: Offset(0),
            length: Length(4),
            format: "u32".to_string(),
            endianness: Endianness::Little,
            ..Default::default()
        };

        let registry = ParserRegistry::new();

        let value_be = registry
            .parse("u32", &big_endian_bytes, &ingredient_be)
            .unwrap();
        let value_le = registry
            .parse("u32", &little_endian_bytes, &ingredient_le)
            .unwrap();

        assert_eq!(value_be, IngredientValue::U32(0x12345678));
        assert_eq!(value_le, IngredientValue::U32(0x12345678));
    }

    /// FLOATING POINT INTEGRITY:
    /// Verified that f32 reconstruction maintains precision across 
    /// endianness boundaries.
    #[test]
    fn test_f32_endianness() {
        let float_value: f32 = 42.42;
        let be_bytes = float_value.to_be_bytes();
        let le_bytes = float_value.to_le_bytes();

        let ingredient_be = Ingredient {
            name: "f32_be".to_string(),
            offset: Offset(0),
            length: Length(4),
            format: "f32".to_string(),
            endianness: Endianness::Big,
            ..Default::default()
        };

        let ingredient_le = Ingredient {
            name: "f32_le".to_string(),
            offset: Offset(0),
            length: Length(4),
            format: "f32".to_string(),
            endianness: Endianness::Little,
            ..Default::default()
        };

        let registry = ParserRegistry::new();

        let parsed_be = match registry.parse("f32",&be_bytes, &ingredient_be).unwrap() {
            IngredientValue::F32(val) => val,
            other => panic!("Unexpected value: {:?}", other),
        };

        let parsed_le = match registry.parse("f32",&le_bytes, &ingredient_le).unwrap() {
            IngredientValue::F32(val) => val,
            other => panic!("Unexpected value: {:?}", other),
        };

        // Using epsilon comparison to handle floating point precision jitter.
        assert!((parsed_be - float_value).abs() < f32::EPSILON);
        assert!((parsed_le - float_value).abs() < f32::EPSILON);
    }
}

