use serde::Deserialize;

use crate::{
    types::{Length, MagicNumber, Offset},
    ingredient::Ingredient
};

/// The Root Manifest of the RECIPE system.
///
/// `Config` represents the high-level blueprint for a deterministic data stream.
/// It partitions the raw bitstream into two primary jurisdictions: the **Header** /// (Identity/Validation) and the **Body** (Operational Payload).
///
/// This structure is designed to be deserialized from external formats (like XML/JSON),
/// allowing for dynamic protocol redefinition without recompiling the core engine.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// The structural metadata and identity anchor for the packet/schema.
    pub header: Header,
    /// The actual data payload consisting of timed ingredients and recursive entries.
    pub body: Body,
}

/// The Validation Sentinel for a data stream.
///
/// The `Header` defines the physical boundaries and the `MagicNumber` required
/// to initiate a lensing operation. It acts as the "Gatekeeper" for the parser.
#[derive(Deserialize, Debug)]
pub struct Header {
    /// The protocol-specific identifier used to verify schema compatibility.
    pub magic_number: MagicNumber,
    /// The global starting position within the binary blob for this header.
    pub offset: Offset,
    /// The physical extent of the header segment in bytes.
    pub length: Length,
}

/// The Operational Payload Jurisdiction.
///
/// This structure defines how the "Lenses" should be applied to the data segment
/// following the header. It supports both flat metadata (timestamp) and 
/// recursive or iterative data structures (entries).
#[derive(Deserialize, Debug)]
pub struct Body {
    /// The primary temporal anchor for the entire payload.
    /// This uses the `Ingredient` model to define its own parsing rules.
    pub timestamp: Ingredient,
    /// A collection of discrete data units, each with its own jurisdictional boundaries.
    pub entries: Vec<Entry>,
}

/// A localized "Micro-Lens" within a data stream.
///
/// `Entry` represents a specific window of situational awareness. It defines 
/// an offset-relative data point, allowing the engine to jump precisely 
/// to the location of a value without scanning the entire stream (O(1) access).
#[derive(Deserialize, Debug)]
pub struct Entry {
    /// The absolute or relative displacement where this specific entry begins.
    pub offset: Offset,
    /// Localized temporal metadata for this specific entry.
    pub timestamp: Ingredient,
    /// The actual data point to be extracted and classified.
    pub value: Ingredient,
}

/// Defines the bit-ordering policy for numeric reconstruction.
///
/// ### Deterministic State
/// In high-assurance environments, mixed-endianness can lead to catastrophic 
/// data corruption. This enum ensures that the "Lenses" apply the correct 
/// CPU-level bit-shuffling based on the protocol's definition.
#[derive(Deserialize, Debug, Clone, Default)]
pub enum Endianness {
    /// Standard for X86 and most modern memory models.
    #[default]
    Little,
    /// Standard for Network Byte Order (e.g., TCP/IP).
    Big
}