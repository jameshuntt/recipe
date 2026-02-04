use serde::Deserialize;

/// A type-safe representation of an IEEE 802 48-bit Media Access Control (MAC) address.
///
/// ### Situational Awareness Role
/// Within the `RECIPE` framework, `MacAddress` acts as a physical-layer anchor. 
/// It ensures that hardware-level identifiers are strictly validated as 6-byte 
/// arrays, preventing buffer overruns or misinterpretation of network frame headers.
///
/// By wrapping the raw `[u8; 6]`, we provide a semantic boundary that distinguishes 
/// hardware addresses from arbitrary binary blobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct MacAddress(pub [u8; 6]);

impl From<[u8; 6]> for MacAddress {
    /// Seamlessly upgrades a raw 6-byte array into a validated `MacAddress` container.
    ///
    /// This is typically invoked by the `network` module parsers when a 
    /// MAC-format ingredient is encountered in the bitstream.
    fn from(bytes: [u8; 6]) -> Self {
        MacAddress(bytes)
    }
}

impl std::fmt::Display for MacAddress {
    /// Formats the hardware address into the standard industrial colon-hexadecimal notation.
    ///
    /// Output Example: `00:1A:2B:3C:4D:5E`
    ///
    /// This implementation ensures that even when secrets are "classified," 
    /// hardware identifiers remain human-readable for diagnostic logging 
    /// without compromising the "Lensing" security of the payload itself.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Deconstruct the NewType via pattern matching to access the underlying byte-array.
        let MacAddress(bytes) = *self;
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        )
    }
}