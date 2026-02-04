// -------------------------------------------------------------------------
// SIGNED INTEGER PARSERS (i8 - i64)
// -------------------------------------------------------------------------
// Generated only when the "signed" feature is active. 
// These handle 2's complement reconstruction via the macro.
#[cfg(feature = "signed")]
impl_simple_parser!(I8Parser, I8,  i8,  1);
#[cfg(feature = "signed")]
impl_simple_parser!(I16Parser, I16,  i16,  2);
#[cfg(feature = "signed")]
impl_simple_parser!(I32Parser, I32,  i32,  4);
#[cfg(feature = "signed")]
impl_simple_parser!(I64Parser, I64,  i64,  8);

// -------------------------------------------------------------------------
// UNSIGNED INTEGER PARSERS (u8 - u64)
// -------------------------------------------------------------------------
// Standard bit-stream primitives. Crucial for offsets, lengths, and 
// bit-flags in protocol headers.
#[cfg(feature = "unsigned")]
impl_simple_parser!(U8Parser, U8,  u8,  1);
#[cfg(feature = "unsigned")]
impl_simple_parser!(U16Parser, U16,  u16,  2);
#[cfg(feature = "unsigned")]
impl_simple_parser!(U32Parser, U32,  u32,  4);
#[cfg(feature = "unsigned")]
impl_simple_parser!(U64Parser, U64,  u64,  8);

// -------------------------------------------------------------------------
// FLOATING POINT PARSERS (f32 - f64)
// -------------------------------------------------------------------------
// Implements IEEE 754 reconstruction. These are vital for sensor 
// telemetry and physical coordinate data.
#[cfg(feature = "float")]
impl_simple_parser!(F32Parser, F32,  f32,  4);
#[cfg(feature = "float")]
impl_simple_parser!(F64Parser, F64,  f64,  8);

#[cfg(feature = "unsigned")]
#[cfg(test)]
mod tests {
    use crate::{
        config::{Endianness},
        types::{Length, Offset},
        ingredient::Ingredient,
        ingredient_value::IngredientValue,
        traits::IngredientParser,
        error::RecipeError,
        impl_simple_parser,
    };

    impl_simple_parser!(U8Parser,  U8,  u8,  1);
    impl_simple_parser!(U16Parser, U16, u16, 2);
    impl_simple_parser!(U32Parser, U32, u32, 4);

    fn ingredient(offset: usize, length: usize, endian: Endianness) -> Ingredient {
        Ingredient {
            offset: Offset(offset),
            length: Length(length),
            endianness: endian,
            ..Default::default()
        }
    }

    #[test]
    fn test_u8_parser() {
        let parser = U8Parser;
        let data = &[0xAB];
        let ing = ingredient(0, 1, Endianness::Little);
        let val = parser.parse(data, &ing).unwrap();
        assert_eq!(val, IngredientValue::U8(0xAB));
    }

    /// VERIFICATION: Big vs Little Endian
    ///
    /// These tests ensure that the bit-shuffling logic correctly handles 
    /// multi-byte integers. 
    /// 0x1234 LE -> [0x34, 0x12]
    /// 0x1234 BE -> [0x12, 0x34]
    #[test]
    fn test_u16_parser_le() {
        let parser = U16Parser;
        let data = &[0x34, 0x12]; // 0x1234 in little endian
        let ing = ingredient(0, 2, Endianness::Little);
        let val = parser.parse(data, &ing).unwrap();
        assert_eq!(val, IngredientValue::U16(0x1234));
    }

    #[test]
    fn test_u16_parser_be() {
        let parser = U16Parser;
        let data = &[0x12, 0x34]; // 0x1234 in big endian
        let ing = ingredient(0, 2, Endianness::Big);
        let val = parser.parse(data, &ing).unwrap();
        assert_eq!(val, IngredientValue::U16(0x1234));
    }

    #[test]
    fn test_u32_parser_le() {
        let parser = U32Parser;
        let data = &[0x78, 0x56, 0x34, 0x12]; // 0x12345678 LE
        let ing = ingredient(0, 4, Endianness::Little);
        let val = parser.parse(data, &ing).unwrap();
        assert_eq!(val, IngredientValue::U32(0x12345678));
    }

    #[test]
    fn test_u32_parser_be() {
        let parser = U32Parser;
        let data = &[0x12, 0x34, 0x56, 0x78]; // 0x12345678 BE
        let ing = ingredient(0, 4, Endianness::Big);
        let val = parser.parse(data, &ing).unwrap();
        assert_eq!(val, IngredientValue::U32(0x12345678));
    }

    /// VERIFICATION: Jurisdictional Bounds
    ///
    /// Ensures that the parser refuses to read past the end of the buffer 
    /// (OutOfBounds) or interpret a slice that doesn't match the 
    /// required type width (InvalidLength).
    #[test]
    fn test_bounds_error() {
        let parser = U16Parser;
        let data = &[0x01]; // too short
        let ing = ingredient(0, 2, Endianness::Little);
        let err = parser.parse(data, &ing).unwrap_err();
        matches!(err, RecipeError::OutOfBounds { .. });
    }

    #[test]
    fn test_invalid_length() {
        let parser = U32Parser;
        let data = &[0x00, 0x00, 0x00, 0x00];
        let ing = ingredient(0, 3, Endianness::Big); // mismatch
        let err = parser.parse(data, &ing).unwrap_err();
        matches!(err, RecipeError::InvalidLength { .. });
    }


    #[test]
    fn test_u8_parser_success() {
        let parser = U8Parser;
        let data = &[0x2A, 0x00, 0xFF]; // 0x2A == 42

        let ingredient = Ingredient {
            offset: Offset(0),
            length: Length(1),
            endianness: Endianness::Little, // should not affect u8
            ..Default::default()
        };

        let result = parser.parse(data, &ingredient);
        assert!(result.is_ok());

        match result.unwrap() {
            IngredientValue::U8(v) => assert_eq!(v, 42),
            _ => panic!("Expected IngredientValue::U8"),
        }
    }

    #[test]
    fn test_u8_parser_out_of_bounds() {
        let parser = U8Parser;
        let data = &[0x2A];

        let ingredient = Ingredient {
            offset: Offset(10),
            length: Length(1),
            endianness: Endianness::Little,
            ..Default::default()
        };

        let result = parser.parse(data, &ingredient);
        assert!(result.is_err());

        match result {
            Err(RecipeError::OutOfBounds { .. }) => {}
            _ => panic!("Expected SliceOutOfBounds error"),
        }
    }

    #[test]
    fn test_u8_parser_zero_length() {
        let parser = U8Parser;
        let data = &[0x2A];

        let ingredient = Ingredient {
            offset: Offset(0),
            length: Length(0),
            endianness: Endianness::Little,
            ..Default::default()
        };

        let result = parser.parse(data, &ingredient);
        assert!(result.is_err());

        match result {
            Err(RecipeError::InvalidLength { .. }) => {}
            _ => panic!("Expected InvalidLength error"),
        }
    }
}

