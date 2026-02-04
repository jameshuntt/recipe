// =============================================================
// CORE TYPE DEFINITIONS & EXPORTS
// =============================================================
// This module acts as the central hub for the RECIPE type system.
// By re-exporting these primitives, we ensure a unified language
// for spatial and identity logic across the entire crate.
// =============================================================

pub mod length;
pub use length::Length;

pub mod magic_number;
pub use magic_number::MagicNumber;

pub mod offset;
pub use offset::Offset;

pub mod mac_address;
pub use mac_address::MacAddress;

use crate::traits::IngredientParser;

/// The "Universal Parser" Type Alias.
///
/// This represents a thread-safe, heap-allocated dynamic trait object.
///
/// ### Concurrency Guarantees:
/// - `Send`: Allows the parser to be moved across thread boundaries.
/// - `Sync`: Allows multiple threads to access the parser simultaneously.
///
/// This is critical for high-throughput environments (like your QUIC 
/// or 500k LoC backend) where multiple packets are parsed in parallel 
/// using a shared Registry.
pub type ParserTrait = dyn IngredientParser + Send + Sync;

use std::collections::HashMap;
use crate::ingredient_value::IngredientValue;
use crate::error::RecipeError;

/// A standardized Result wrapper for the RECIPE ecosystem.
///
/// Ensures that all operations within the crate converge on a 
/// deterministic error domain (`RecipeError`), facilitating 
/// simpler error propagation using the `?` operator.
pub type RecipeResult<T> = Result<T, RecipeError>;

/// Represents the final output of a completed Packet Lens operation.
///
/// Returns a map where:
/// - **Key**: The semantic name of the field (from the Blueprint).
/// - **Value**: The classified and extracted data.
pub type PacketResult = RecipeResult<HashMap<String, IngredientValue>>;

/// Represents the output of a single Ingredient extraction.
///
/// Used by individual parsers within the registry to return a 
/// successfully coerced value or a specific jurisdictional error.
pub type IngredientValueResult = RecipeResult<IngredientValue>;