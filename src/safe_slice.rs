#![allow(unused)]

use crate::error::RecipeError;

pub(crate) fn safe_slice(
    data: &[u8],
    offset: usize,
    length: usize
) -> Result<&[u8], RecipeError> {
    if offset + length > data.len() {
        return Err(RecipeError::OutOfBounds);
    }
    Ok(&data[offset..offset + length])
}