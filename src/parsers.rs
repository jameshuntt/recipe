// --- Core Primitives ---
// Numerical parsers (u8-u64, i8-i64, f32-f64) are considered the 
// foundational "Ingredients" of the engine.
pub mod numerical;
pub use numerical::*;

// --- Container Parsers ---
// These allow for hierarchical data structures. They are gated behind 
// the "composite" feature to reduce complexity for flat-file parsers.
#[cfg(feature = "composite")]
pub mod array_parser;
#[cfg(feature = "composite")]
pub use array_parser::*;

#[cfg(feature = "composite")]
pub mod struct_parser;
#[cfg(feature = "composite")]
pub use struct_parser::*;

// --- Network Identity Parsers ---
// Specialized parsers for Layer 2 (MAC) and Layer 3 (IP) protocols.
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "network")]
pub use network::*;

// --- Utility & Helpers ---
// Shared logic for boundary calculation and result accumulation.
pub mod helpers;
pub use helpers::*;

// --- Variable Length Parsers ---
// Standard parsers for text (UTF-8) and opaque binary blobs.
pub mod string_parser;
pub use string_parser::*;
pub mod binary_parser;
pub use binary_parser::*;