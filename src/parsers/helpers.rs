use std::{collections::HashMap};
use crate::{
    config::Config,
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue,
    parser_registry::ParserRegistry,
    types::{Length, PacketResult, RecipeResult}
};

/// A specialized utility for quick IPv4 string reconstruction.
/// 
/// Uses the `require_length_for!` macro to enforce a strict 4-byte boundary
/// before attempting to join octets into a standard dotted-decimal string.
pub fn parse_ipv4_basic(data: &[u8], offset: usize, length: usize) -> RecipeResult<String> {
    require_length_for!(length, 4, ipv4);

    let octets = &data[offset..offset + length];
    Ok(octets.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."))
}

/// Computes the safe slicing boundaries for a given ingredient.
/// 
/// ### Security Logic:
/// It implements a "Clamp" mechanism: the end boundary is the minimum of 
/// the requested length and the `max_length` safety ceiling.
/// It also performs an explicit check against the physical `data` length 
/// to prevent `OutOfBounds` panics.
pub fn calculate_bounds(data: &[u8], ingredient: &Ingredient) -> RecipeResult<(usize, usize)> {
    let start = ingredient.offset.0;
    
    // Safety: Protect against 'Greedy' length definitions in the config.
    let end = start + ingredient.length.0.min(
        ingredient.max_length.unwrap_or(Length(usize::MAX)).0
    );
    
    if end > data.len() {
        return Err(RecipeError::OutOfBounds);
    }
    Ok((start, end))
}

/// The core recursive dispatcher for populating the results map.
/// 
/// This function acts as the bridge between the `ParserRegistry` and the 
/// final `HashMap`. It handles the namespacing (prefixing) logic to ensure
/// nested fields have unique keys (e.g., "entry_1.status").
pub fn parse_table(
    data: &[u8], 
    ingredient: &Ingredient, 
    prefix: &str, 
    results: &mut HashMap<String, IngredientValue>
) -> RecipeResult<()> {
    // Note: Instantiating the registry here ensures we have the latest
    // feature-gated parsers available for this specific lens operation.
    let r = ParserRegistry::new();
    
    // Construct the dot-notated path key.
    let key = if prefix.is_empty() {
        ingredient.name.clone()
    } else {
        format!("{}.{}", prefix, ingredient.name)
    };

    // DISPATCH LOGIC:
    // We treat 'array' and 'struct' as recursive containers, 
    // while all other formats are treated as leaf-node primitives.
    match ingredient.format.as_str() {
        "array" => {
            let array_result = r.parse_ingredient(data, ingredient)?;
            results.insert(key, array_result);
        }
        "struct" => {
            let struct_result = r.parse_ingredient(data, ingredient)?;
            results.insert(key, struct_result);
        }
        _ => {
            let ingredient_value = r.parse_ingredient(data, ingredient)?;
            results.insert(key, ingredient_value);
        }
    }
    Ok(())
}

/// High-level packet orchestrator.
/// 
/// Iterates through the config entries and triggers a deep-dive parse
/// for every child ingredient defined within the entry value.
pub fn parse_packet(data: &[u8], config: &Config) -> PacketResult {
    let mut results = HashMap::new();
    let entries = config.body.entries.iter().clone();
    
    for entry in entries {
        // Validation: Ensure the entry value is actually a container (struct).
        let child_ingredients = entry.value.children.as_ref()
            .ok_or(RecipeError::InvalidConfig("No child ingredients defined for struct".to_string()))?;

        for child in child_ingredients {
            // Recurse into the table with the child's specific name as the prefix.
            parse_table(data, child, &child.name, &mut results)?;
        }
    }
    
    Ok(results)
}

/// Sequentially numbered packet orchestrator.
/// 
/// Similar to `parse_packet`, but automatically namespaces the results 
/// using the index of the entry (e.g., "entry_0", "entry_1").
/// 
/// This is the preferred method for parsing repeating logs or telemetry frames
/// where individual entry names are not provided in the schema.
pub fn parse_packet_numbered(data: &[u8], config: &Config) -> PacketResult {
    let mut results = HashMap::new();

    for (i, entry) in config.body.entries.iter().enumerate() {
        let ingredient = &entry.value;

        // Apply a numerical prefix to prevent key collisions in the HashMap.
        parse_table(data, ingredient, &format!("entry_{}", i), &mut results)?;
    }

    Ok(results)
}