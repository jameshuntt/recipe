use serde::Deserialize;
use std::collections::HashMap;
#[cfg(feature = "network")]
use std::net::{Ipv4Addr, Ipv6Addr};

#[cfg(feature = "network")]
use crate::types::MacAddress;

/// The Universal Data Container for the RECIPE ecosystem.
///
/// `IngredientValue` is the final product of a Lensing operation. It acts as a 
/// "Sum Type" (Enum) that allows the engine to hold vastly different data 
/// types—from a single bit to a complex recursive struct—within a single 
/// unified interface.
///
/// ### Feature-Gated Architecture
/// To maintain a minimal memory footprint in constrained environments (e.g., 
/// embedded systems or high-frequency trading taps), this enum uses 
/// conditional compilation (`#[cfg]`). You only pay the memory cost for 
/// the types your specific "Recipe" actually uses.
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum IngredientValue {
    // -------------------------------------------------------------------------
    // SIGNED NUMERICS: Integer math with polarity.
    // -------------------------------------------------------------------------
    #[cfg(feature = "signed")] I8(i8),
    #[cfg(feature = "signed")] I16(i16),
    #[cfg(feature = "signed")] I32(i32),
    #[cfg(feature = "signed")] I64(i64),

    // -------------------------------------------------------------------------
    // UNSIGNED NUMERICS: Raw bit counts and hardware registers.
    // -------------------------------------------------------------------------
    #[cfg(feature = "unsigned")] U8(u8),
    #[cfg(feature = "unsigned")] U16(u16),
    #[cfg(feature = "unsigned")] U32(u32),
    #[cfg(feature = "unsigned")] U64(u64),

    // -------------------------------------------------------------------------
    // FLOATING POINT: Scientific or sensor data.
    // -------------------------------------------------------------------------
    #[cfg(feature = "float")] F32(f32),
    #[cfg(feature = "float")] F64(f64),

    // -------------------------------------------------------------------------
    // NETWORK PRIMITIVES: Physical and Logical Addressing.
    // -------------------------------------------------------------------------
    #[cfg(feature = "network")] IpV4(Ipv4Addr),
    #[cfg(feature = "network")] IpV6(Ipv6Addr),
    #[cfg(feature = "network")] Ipv4Str(String),
    #[cfg(feature = "network")] Ipv6Str(String),
    #[cfg(feature = "network")] Mac(MacAddress),

    // -------------------------------------------------------------------------
    // COMPOSITE TYPES: Recursive Data Structures.
    // -------------------------------------------------------------------------
    /// Represents a repeating list of ingredients (e.g., a packet log).
    #[cfg(feature = "composite")] Array(Vec<IngredientValue>),
    /// Represents a nested object/schema mapping names to values.
    #[cfg(feature = "composite")] Struct(HashMap<String, IngredientValue>),

    // -------------------------------------------------------------------------
    // FALLBACK PRIMITIVES: Arbitrary Data.
    // -------------------------------------------------------------------------
    /// A raw byte buffer used for unclassified or encrypted payloads.
    Binary(Vec<u8>),
    /// Human-readable text data.
    String(String),
}

impl IngredientValue {
    // =========================================================================
    // TYPE-SAFE ACCESSOR INTERFACE
    // =========================================================================
    // These methods provide a clean way to extract data without manual 
    // pattern matching. They utilize a macro-driven approach to ensure 
    // consistency across the type system.
    // =========================================================================

    #[cfg(feature = "signed")]
    impl_accessor!(as_i8, i8, IngredientValue::I8);
    #[cfg(feature = "signed")]
    impl_accessor!(as_i16, i16, IngredientValue::I16);
    #[cfg(feature = "signed")]
    impl_accessor!(as_i32, i32, IngredientValue::I32);
    #[cfg(feature = "signed")]
    impl_accessor!(as_i64, i64, IngredientValue::I64);
    
    #[cfg(feature = "unsigned")]
    impl_accessor!(as_u8, u8, IngredientValue::U8);
    #[cfg(feature = "unsigned")]
    impl_accessor!(as_u16, u16, IngredientValue::U16);
    #[cfg(feature = "unsigned")]
    impl_accessor!(as_u32, u32, IngredientValue::U32);
    #[cfg(feature = "unsigned")]
    impl_accessor!(as_u64, u64, IngredientValue::U64);
    
    #[cfg(feature = "float")]
    impl_accessor!(as_f32, f32, IngredientValue::F32);
    #[cfg(feature = "float")]
    impl_accessor!(as_f64, f64, IngredientValue::F64);
    
    #[cfg(feature = "network")]
    impl_accessor!(as_ipv4, Ipv4Addr, IngredientValue::IpV4);
    #[cfg(feature = "network")]
    impl_accessor!(as_ipv6, Ipv6Addr, IngredientValue::IpV6);
    
    #[cfg(feature = "composite")]
    // /// Attempts to view the value as a list of nested ingredients.
    impl_accessor!(as_array, Vec<IngredientValue>, IngredientValue::Array);
    #[cfg(feature = "composite")]
    // /// Attempts to view the value as a structured object map.
    impl_accessor!(as_struct, HashMap<String, IngredientValue>, IngredientValue::Struct);
    
    impl_accessor!(as_string, String, IngredientValue::String);
    impl_accessor!(as_binary, Vec<u8>, IngredientValue::Binary);
}