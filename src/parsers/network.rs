// -------------------------------------------------------------------------
// IP ADDRESS PARSERS (Layer 3)
// -------------------------------------------------------------------------
// Generated only when the "network" feature is active. 
// These parsers utilize `std::net` types to ensure the resulting data 
// is immediately compatible with standard Rust networking stacks.

// /// Parses a 4-byte sequence into an Ipv4Addr.
// /// Enforces a strict 32-bit boundary.
#[cfg(feature = "network")]
impl_ip_addr_parser!(IpV4Parser, IpV4, ::std::net::Ipv4Addr, 4);

// /// Parses a 16-byte sequence into an Ipv6Addr.
// /// Essential for modern high-address-space environments.
#[cfg(feature = "network")]
impl_ip_addr_parser!(IpV6Parser, IpV6, ::std::net::Ipv6Addr, 16);

// -------------------------------------------------------------------------
// HARDWARE ADDRESS PARSER (Layer 2)
// -------------------------------------------------------------------------

// /// The MAC Address Resolver.
// /// 
// /// Uses `impl_fixed_array_parser` to capture exactly 6 bytes.
// /// This is used for physical device identification (EUI-48). 
// /// Unlike IPs, MAC addresses are treated as fixed byte-arrays to preserve 
// /// the OUI (Organizationally Unique Identifier) formatting.
#[cfg(feature = "network")]
impl_fixed_array_parser!(MacParser, Mac, crate::types::MacAddress, 6);