use crate::{
    types::{Length, Offset},
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_value::IngredientValue
};

/// The fundamental interface for data extraction.
///
/// `IngredientParser` defines the contract for all "Lensing" operations. 
/// Any structure implementing this trait gains the capability to transform 
/// raw bitstreams into high-level, classified data structures.
pub trait IngredientParser {
    /// Attempts to interpret a slice of the physical buffer based on 
    /// the jurisdictional rules of the provided `Ingredient`.
    ///
    /// ### Implementation Requirements:
    /// - Must be deterministic.
    /// - Must handle endianness according to the `Ingredient` manifest.
    /// - Must return a `RecipeError` if the data does not conform to the expected format.
    fn parse(
        &self,
        data: &[u8],
        ingredient: &Ingredient
    ) -> Result<IngredientValue, RecipeError>;
}

/// A specialized utility trait for spatial validation.
///
/// `CalculateBounds` provides a unified logic for determining the 
/// 'Operational Window' of a parser. It ensures that the "Situational Awareness" 
/// of the lens is strictly clamped within physical memory limits.
pub trait CalculateBounds {
    /// Computes the start and end indices for a memory operation.
    ///
    /// ### Safety Logic:
    /// This method enforces a "Double-Clamp":
    /// 1. It honors the `offset`.
    /// 2. It respects the `length` but caps it at the `max_length` (if provided).
    /// 3. It performs a final safety check against the actual `data.len()` to 
    ///    prevent hardware-level memory violations (Panics).
    fn calculate_bounds(data: &[u8], ingredient: &Ingredient) -> Result<(usize, usize), RecipeError> {
        let start = ingredient.offset.0;
        
        // JURISDICTIONAL CLAMPING: 
        // We take the requested length but never exceed the hard maximum 
        // allowed by the Blueprint.
        let end = start + ingredient.length.0.min(
            ingredient.max_length.unwrap_or(Length(usize::MAX)).0
        );

        // BOUNDARY ENFORCEMENT:
        // Final sanity check. If the 'end' jurisdiction falls outside 
        // the physical slice, we throw a Spatial Violation (OutOfBounds).
        if end > data.len() {
            return Err(RecipeError::OutOfBounds);
        }
        Ok((start, end))
    }
}

/// Defines the physical footprint of a structure.
///
/// Implementing `HasLayout` allows the `RECIPE` engine to reason about 
/// where a structure "lives" in the bitstream, enabling automatic 
/// offset calculation and recursive layout planning.
pub trait HasLayout {
    /// Returns the absolute or relative starting point.
    fn offset(&self) -> Offset;
    /// Returns the spatial extent (size) of the structure.
    fn length(&self) -> Length;
}