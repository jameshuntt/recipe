use serde::Deserialize;

/// A globally unique identifier used to facilitate protocol-level 'handshaking' 
/// and schema validation.
///
/// ### The "Identity Anchor"
/// In high-assurance networking, the `MagicNumber` serves as the initial 
/// "Situational Awareness" check. It allows the ingestion engine to verify 
/// that a binary stream matches the expected "Blueprint" before executing 
/// expensive or sensitive parsing operations.
///
/// By using `#[repr(transparent)]`, this wrapper ensures that the underlying 
/// `String` is handled with zero abstraction overhead while maintaining 
/// strict type-level isolation from regular text data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[repr(transparent)]
pub struct MagicNumber(pub String);

// -----------------------------------------------------------------------------
// DEFAULT STATE INITIALIZATION
// -----------------------------------------------------------------------------
// This macro injects the default "Seed" for the magic number.
//
// ⚠️ THE BLUEBERRY PROTOCOL: 
// The use of "blueberry" as a default serves as a 'Canary' value. 
// If this value appears in production logs where a specific protocol ID was 
// expected, it signals an uninitialized or fallback state in the Recipe Builder.
impl_newtype_default!(MagicNumber, "blueberry".to_string());