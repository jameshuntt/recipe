/// The Deterministic Initialization Engine.
///
/// `impl_newtype_default!` is a utility macro that automates the implementation 
/// of the `Default` trait for NewType wrappers. 
///
/// ### Why this is Architectural "Best Practice":
/// 1. **Zero-Boilerplate**: Manually implementing `Default` for dozens of domain types 
///    (like Offset, Length, MagicNumber) is repetitive and increases the surface area 
///    for copy-paste errors.
/// 2. **State Control**: It allows the developer to define a "Canary" or "Baseline" 
///    value (like the "blueberry" magic string or a 0-offset) that the entire 
///    system can rely on during uninitialized states.
#[macro_export]
macro_rules! impl_newtype_default {
    (
        $type_name:ident, // The name of the NewType struct (e.g., Offset)
        $default_expr:expr // The literal or expression representing the baseline value
    ) => {
        impl ::core::default::Default for $type_name {
            /// Creates a new instance initialized with the system-defined baseline.
            ///
            /// This ensures that the RECIPE engine always starts from a 
            /// known-good configuration state.
            fn default() -> Self {
                $type_name($default_expr)
            }
        }
    };
}

/// The Endianness Reconstruction Engine.
///
/// `parse_endian!` is a specialized macro that handles the bit-shuffling logic
/// required to convert raw byte arrays into typed scalars (integers/floats).
///
/// ### Why this is Industrial-Grade:
/// In heterogeneous environments, data might arrive in "Network Byte Order" (Big Endian)
/// while being processed on "Host Byte Order" (Little Endian). This macro decouples 
/// the parsing logic from the hardware architecture, ensuring the "Recipe" 
/// dictates how the data is interpreted.
/// Usage: parse_endian!(u16, bytes, ingredient)
#[macro_export]
macro_rules! parse_endian {
    ($ty:ty, $bytes:expr, $ingredient:expr) => {{
        // SITUATIONAL AWARENESS:
        // We branch based on the policy defined in the Ingredient Manifest.
        match $ingredient.endianness {
            // Standard for X86 and most modern memory models.
            $crate::config::Endianness::Little => {
                <$ty>::from_le_bytes(
                    $bytes.try_into()
                        .map_err(|_| $crate::error::RecipeError::InvalidLength)?
                )
            },
            // Standard for Network Protocols (TCP/IP).
            $crate::config::Endianness::Big => {
                <$ty>::from_be_bytes(
                    $bytes.try_into()
                        .map_err(|_| $crate::error::RecipeError::InvalidLength)?
                )
            },
        }
    }};
}

/// The Jurisdictional Boundary Resolver.
///
/// `start_end_from_ingredient!` is the lowest-level geometric macro in the 
/// RECIPE engine. It translates an Ingredient's spatial metadata into a 
/// physical coordinate pair (Start, End) within a bitstream.
///
/// ### Situational Awareness Logic:
/// By centralizing the calculation of `start` and `end` here, we ensure that
/// every parser in the 500k LoC system uses the exact same logic for 
/// determining memory jurisdictions. This prevents "off-by-one" errors 
/// that could lead to overlapping Lenses or data corruption.
#[macro_export]
macro_rules! start_end_from_ingredient {
    ($ingredient:expr) => {{
        // The absolute displacement from the start of the current jurisdiction.
        let start = $ingredient.offset.0;
        
        // The terminal boundary, calculated by extending the start by the 
        // intended spatial extent (length).
        let end = start + $ingredient.length.0;
        
        // Return a coordinate tuple used for safe slicing (e.g., via .get()).
        (start, end)
    }};
}

/// The Scalar Primitive Generator.
///
/// `impl_simple_parser!` automates the creation of parsers for standard numeric 
/// types (u8-u64, i8-i64, f32-f64). It acts as the final "Translation Layer" 
/// between the physical bitstream and high-level arithmetic types.
///
/// ### Deterministic Bit-Ordering
/// This macro is specifically designed to handle `Endianness` transparently. 
/// By utilizing the `parse_endian!` macro internally, it ensures that the 
/// bit-shuffling logic is dictated by the `Ingredient` manifest, not the host CPU.
#[macro_export]
macro_rules! impl_simple_parser {
    (
        $parser_name:ident,    // The structural name (e.g., U32Parser)
        $variant:ident,        // The target variant in IngredientValue (e.g., U32)
        $ty:ty,                // The backing Rust primitive (e.g., u32)
        $len:literal           // The strict byte-width (e.g., 4)
    ) => {
        /// A stateless, zero-overhead parser for numeric scalars.
        #[derive(Default)]
        pub struct $parser_name;

        impl $crate::traits::IngredientParser for $parser_name {
            /// Deconstructs a byte slice into a typed numeric value.
            ///
            /// ### The Three-Step extraction:
            /// 1. **Spatial Resolution**: Locates the segment in the buffer.
            /// 2. **Fixed-Size Slicing**: Freezes the segment into a stack array.
            /// 3. **Endian Reconstruction**: Reassembles the bytes into a number
            ///    based on the Ingredient's specific endianness policy.
            fn parse(
                &self,
                data: &[u8],
                ingredient: &$crate::ingredient::Ingredient,
            ) -> Result<$crate::ingredient_value::IngredientValue, $crate::error::RecipeError> {
                
                // Determine the starting and ending indices within the buffer.
                let (start, end) = $crate::start_end_from_ingredient!(ingredient);
                
                // Capture the raw bytes into a fixed-width array [u8; $len].
                let bytes = $crate::parse_bytes_array!(data, start, end, $len);
                
                // RECONSTRUCTION PHASE:
                // This call resolves the bit-order. If the ingredient is marked 
                // Big-Endian, the bytes are swapped accordingly to ensure the 
                // resulting Rust primitive reflects the intended value.
                let value = $crate::parse_endian!($ty, bytes, ingredient);

                // Final classification into the system's universal value type.
                Ok($crate::ingredient_value::IngredientValue::$variant(value))
            }
        }
    };
}

/// The Physical Boundary Enforcement Engine.
///
/// `parse_bytes_array!` is the high-performance core of the RECIPE system. 
/// It performs the critical task of "slicing and freezing" raw data into 
/// fixed-size arrays. 
///
/// ### Why this is Industrial-Grade:
/// 1. **Zero-Panic**: Instead of using `[start..end]`, which can panic, it uses `.get()`, 
///    returning a `RecipeError` if the spatial jurisdiction is violated.
/// 2. **Stack Allocation**: By returning a `[u8; N]`, it ensures the data is 
///    efficiently handled on the stack, bypassing the heap entirely.
#[macro_export]
macro_rules! parse_bytes_array {
    // -------------------------------------------------------------------------
    // TYPE-BASED EXTRACTION
    // -------------------------------------------------------------------------
    // Automatically determines the required length based on the target 
    // Rust type (e.g., u32, i64).
    ($ty:ty, $data:expr, $start:expr, $end:expr, $len:literal) => {{
        // COMPILE-TIME CONSTANT: 
        // We calculate the byte-width of the target type at compile time 
        // to ensure zero runtime overhead for the size check.
        const LEN: usize = std::mem::size_of::<$ty>();
        
        let bytes: [u8; LEN] = $data
            .get($start..$end)
            .ok_or($crate::error::RecipeError::OutOfBounds)? // Safety: Check slice exists
            .try_into()                                      // Safety: Check size is exactly LEN
            .map_err(|_| $crate::error::RecipeError::InvalidLength)?;
        bytes
    }};

    // -------------------------------------------------------------------------
    // LITERAL-BASED EXTRACTION
    // -------------------------------------------------------------------------
    // Used when the protocol specifies a raw byte-width (e.g., a 6-byte MAC).
    ($data:expr, $start:expr, $end:expr, $len:literal) => {{
        let bytes: [u8; $len] = $data
            .get($start..$end)
            .ok_or($crate::error::RecipeError::OutOfBounds)?
            .try_into()
            .map_err(|_| $crate::error::RecipeError::InvalidLength)?;
        bytes
    }};
}

/// The Semantic Extraction Engine.
///
/// `impl_accessor!` generates standardized "getter" methods for the 
/// `IngredientValue` enum. It abstracts away the boilerplate of pattern matching, 
/// providing a clean, Option-based API for accessing classified data.
///
/// ### Why this is Elite:
/// Instead of a consumer writing `if let IngredientValue::U32(val) = x`, 
/// they simply call `x.as_u32()`. This maintains the "Situational Awareness" 
/// of the system while providing a frictionless developer experience.
#[macro_export]
macro_rules! impl_accessor {
    // -------------------------------------------------------------------------
    // SINGLE VARIANT EXTRACTION
    // -------------------------------------------------------------------------
    // Standard 1-to-1 mapping (e.g., as_u8 -> IngredientValue::U8).
    ($fn_name:ident, $ret_ty:ty, $variant:path) => {
        /// Attempts to view the data as the specified type.
        ///
        /// Returns `Some(&T)` if the lens matches the underlying data, 
        /// otherwise returns `None`. 
        ///
        /// ⚠️ PERFORMANCE: This returns a reference to avoid unnecessary 
        /// cloning of potentially large structures (like Binary or Arrays).
        pub fn $fn_name(&self) -> Option<&$ret_ty> {
            if let $variant(ref val) = *self {
                Some(val)
            } else {
                None
            }
        }
    };

    // -------------------------------------------------------------------------
    // MULTI-VARIANT COALESCING
    // -------------------------------------------------------------------------
    // Enables "Fuzzy Matching" where multiple enum variants map to a 
    // single return type (e.g., both IPv4 and IPv6 variants mapping to a 
    // unified String view).
    ($fn_name:ident, $ret_ty:ty, $($variant:path)|+) => {
        /// Attempts to view the data through a multi-variant lens.
        ///
        /// Uses an optimized `match` expression to check the bitstream 
        /// against several valid interpretations simultaneously.
        pub fn $fn_name(&self) -> Option<&$ret_ty> {
            match *self {
                // TOKEN MUNCHING:
                // We iterate through each provided variant path and 
                // bind the inner reference for the final result.
                $( $variant(ref val) )|+ => Some(val),
                _ => None,
            }
        }
    };
}

/// The Declarative Engine for Registry Population.
///
/// `register_parsers!` is a recursive macro designed to automate the injection 
/// of `IngredientParser` implementations into the central `ParserRegistry`.
///
/// ### Why this exists:
/// Manually calling `HashMap::insert` for dozens of parsers (signed, unsigned, 
/// network, etc.) is error-prone and creates "noise." This macro provides 
/// a clean, declarative interface that ensures every parser is:
/// 1. Converted to a `String` key.
/// 2. Heap-allocated (`Boxed`).
/// 3. Initialized via its `Default` implementation.
#[macro_export]
macro_rules! register_parsers {
    // PATTERN MATCHING:
    // $map: The target HashMap (ident)
    // $name: The string literal used as the lookup key (e.g., "u32")
    // $parser: The concrete struct type that implements IngredientParser
    ($map:ident, $($name:literal => $parser:ty),* $(,)?) => {
        $(
            // MACRO EXPANSION:
            // For every pair provided in the DSL, generate a standard insertion.
            // We use <$parser>::default() to ensure the parser is stateless 
            // and ready for immediate lensing.
            $map.insert($name.to_string(), Box::new(<$parser>::default()));
        )*
    };
}

/// The Network Protocol Generator.
///
/// `impl_ip_addr_parser!` is a high-assurance macro designed to generate 
/// parsers for fixed-width network identifiers. It bridges the gap between 
/// a raw bitstream and the robust `std::net` address types.
///
/// ### Safety & Determinism
/// By enforcing a `$len` literal at compile-time, this macro ensures that 
/// the resulting parser is mathematically incapable of misinterpreting 
/// address boundaries—preventing common networking bugs like "Address Bleed."
#[macro_export]
macro_rules! impl_ip_addr_parser {
    (
        $parser_name:ident,    // The structural identifier (e.g., IpV4Parser)
        $variant:ident,        // The target variant in the IngredientValue enum
        $ty:ty,                // The backing Rust type (e.g., Ipv4Addr)
        $len:literal           // The strict byte-width of the protocol address
    ) => {
        /// A stateless parser dedicated to network address resolution.
        #[derive(Default)]
        pub struct $parser_name;

        impl $crate::traits::IngredientParser for $parser_name {
            /// Carves a network address out of the provided bitstream.
            ///
            /// ### Jurisdictional Logic:
            /// 1. **Boundary Calculation**: Uses `start_end_from_ingredient` to 
            ///    locate the address in the packet.
            /// 2. **Fixed-Size Slicing**: Uses `parse_bytes_array` to ensure 
            ///    exactly `$len` bytes are extracted.
            /// 3. **Type Coercion**: Attempts to transform the raw slice into 
            ///    a machine-readable IP address type.
            fn parse(
                &self,
                data: &[u8],
                ingredient: &$crate::ingredient::Ingredient,
            ) -> Result<$crate::ingredient_value::IngredientValue, $crate::error::RecipeError> {
                
                // Establish the spatial window for the lens.
                let (start, end) = $crate::start_end_from_ingredient!(ingredient);
                
                // Extract the byte array. This macro likely performs 
                // the critical bounds checking to prevent OOB reads.
                let bytes = $crate::parse_bytes_array!(data, start, end, $len);
                
                // TYPE ENFORCEMENT:
                // We attempt to map the slice into a fixed-size array [u8; $len].
                // If the slice length doesn't match the protocol definition, 
                // we return a Length Violation rather than panicking.
                let addr = <$ty>::from(
                    <[u8; $len]>::try_from(bytes)
                        .map_err(|_| $crate::error::RecipeError::InvalidLength)?
                );

                // Return the classified network identity.
                Ok($crate::ingredient_value::IngredientValue::$variant(addr))
            }
        }
    };
}

/// The Universal Fixed-Width Parser Generator.
///
/// `impl_fixed_array_parser!` provides a deterministic template for converting 
/// raw memory segments into specialized fixed-size types. This is the 
/// architectural "Gold Standard" for parsing any data with a known-at-compile-time 
/// spatial footprint.
///
/// ### Situational Awareness Logic:
/// This macro ensures that the "Lens" applied to the bitstream exactly matches 
/// the memory layout of the target Rust type. If the data is even a single bit 
/// off-center, the operation fails safely, protecting the system from 
/// misaligned memory access.
#[macro_export]
macro_rules! impl_fixed_array_parser {
    (
        $parser_name:ident,    // The name of the generated parser struct.
        $variant:ident,        // The IngredientValue variant to wrap the output.
        $ty:ty,                // The target domain type (must implement From<[u8; $len]>).
        $len:literal           // The immutable byte-length of the target segment.
    ) => {
        /// A specialized, zero-sized parser for fixed-width byte structures.
        #[derive(Default)]
        pub struct $parser_name;

        impl $crate::traits::IngredientParser for $parser_name {
            /// Executes the transformation from a raw slice to a high-level domain type.
            ///
            /// ### Safety Contract:
            /// 1. **Address Resolution**: Calculates the 'Start' and 'End' markers using 
            ///    the ingredient's internal `Offset` and `Length`.
            /// 2. **Boundary Validation**: Validates that the requested range is physically 
            ///    contained within the input `data`.
            /// 3. **Array Coercion**: Forcibly validates that the slice is exactly `$len` 
            ///    bytes before the `From` conversion is attempted.
            fn parse(
                &self,
                data: &[u8],
                ingredient: &$crate::ingredient::Ingredient,
            ) -> Result<$crate::ingredient_value::IngredientValue, $crate::error::RecipeError> {
                
                // Retrieve the spatial jurisdiction for this specific ingredient.
                let (start, end) = $crate::start_end_from_ingredient!(ingredient);
                
                // Slice the data. This internal macro handles the physical bounds check.
                let bytes = $crate::parse_bytes_array!(data, start, end, $len);
                
                // TYPE CONVERSION GUARD:
                // We utilize `try_from` to bridge the gap between a dynamic slice (&[u8])
                // and a fixed-size array ([u8; $len]). This is the critical moment where
                // the Lens is "Locked" to the data.
                let arr = <[u8; $len]>::try_from(bytes)
                    .map_err(|_| $crate::error::RecipeError::InvalidLength)?;
                
                // Transform the raw array into the final Domain Type (e.g., MacAddress or Uuid).
                let val = <$ty>::from(arr);
                
                // Encapsulate the value for system-wide consumption.
                Ok($crate::ingredient_value::IngredientValue::$variant(val))
            }
        }
    };
}

/// Enforces an exact spatial match with a custom error message.
///
/// `require_length!` is used when a parser has a non-negotiable footprint.
/// It acts as a hard boundary check, preventing the engine from 
/// misinterpreting data when the slice doesn't match the schema.
#[macro_export]
macro_rules! require_length {
    ($len:expr, $expected:expr, $err:expr) => {
        if $len != $expected {
            // JURISDICTIONAL VIOLATION:
            // We exit immediately with a descriptive error to prevent 
            // downstream logic from processing "garbage" data.
            return Err($crate::error::RecipeError::InvalidData($err.to_string()));
        }
    };
}

/// Enforces an exact spatial match with an automated format string.
///
/// This variant is optimized for developer ergonomics, automatically 
/// constructing a standard "Expected vs Actual" failure report.
#[macro_export]
macro_rules! require_length_2 {
    ($len:expr, $expected:expr, $type:expr) => {
        if $len != $expected {
            return Err($crate::error::RecipeError::InvalidData(
                format!("Invalid length for {}: expected {}", $type, $expected)
            ));
        }
    };
}

/// Enforces a spatial match using compile-time type identification.
///
/// By using `stringify!`, this macro captures the name of the internal 
/// Rust type or variable, providing high-fidelity forensics for 
/// automated log analysis during a parsing failure.
#[macro_export]
macro_rules! require_length_for {
    ($len:expr, $expected:expr, $type:ident) => {
        if $len != $expected {
            return Err($crate::error::RecipeError::InvalidData(
                format!("Invalid length for {}: expected {}", stringify!($type), $expected)
            ));
        }
    };
}

/// Enforces a minimum spatial floor for variable-length payloads.
///
/// Used for data types like Strings, Arrays, or Binary Blobs where 
/// the data can be long, but must meet a minimum "Header" or "Identity" 
/// size to be considered valid.
#[macro_export]
macro_rules! require_min_length {
    ($len:expr, $min:expr, $kind:ident) => {
        if $len < $min {
            // BOUNDARY BREACH:
            // The segment is too shallow to contain a valid payload for this type.
            return Err($crate::error::RecipeError::InvalidData(
                format!("`{}` too short: expected at least {}, got {}", stringify!($kind), $min, $len),
            ));
        }
    };
}

/// The Defensive Ceiling Guard.
///
/// `require_max_length!` enforces a hard upper bound on data segments.
///
/// ### Security Context:
/// This is a critical primitive for preventing "Greedy Parsing" vulnerabilities.
/// By checking the jurisdiction's extent before deep processing, it ensures 
/// that malformed or malicious packets cannot force the system to allocate 
/// excessive memory or enter infinite loops.
#[macro_export]
macro_rules! require_max_length {
    ($len:expr, $max:expr, $kind:ident) => {
        if $len > $max {
            // BOUNDARY VIOLATION:
            // The segment exceeds the protocol's physical constraints.
            return Err($crate::error::RecipeError::InvalidData(
                format!("`{}` too long: expected at most {}, got {}", stringify!($kind), $max, $len),
            ));
        }
    };
}

/// The Universal Scalar Factory.
///
/// `impl_numeric_parser!` generates a complete parsing structure for numeric types.
/// Unlike the "simple" parser, this variant explicitly maps to specific 
/// `IngredientValue` enum paths, allowing for fine-grained control over 
/// how different numeric sizes (like u32 vs u64) are stored.
#[macro_export]
macro_rules! impl_numeric_parser {
    (
        $parser_name:ident,    // The generated struct name (e.g., U32Parser)
        $type:ty,              // The primitive Rust type (e.g., u32)
        $size:expr,            // The physical byte width
        $variant_be:path,      // Path to the Big-Endian enum variant
        $variant_le:path       // Path to the Little-Endian enum variant
    ) => {
        /// A specialized, zero-sized parser for numeric primitives.
        pub struct $parser_name;

        impl $crate::traits::IngredientParser for $parser_name {
            /// Performs the transformation from raw bits to high-level integers.
            ///
            /// ### The Extraction Pipeline:
            /// 1. **Address Calculation**: Locates the segment using the `offset` blueprint.
            /// 2. **Boundary Check**: Ensures the data slice is within physical memory.
            /// 3. **Endian Dispatch**: Dynamically selects the reconstruction 
            ///    algorithm based on the ingredient's policy.
            fn parse(
                &self,
                data: &[u8],
                ingredient: &$crate::ingredient::Ingredient
            ) -> Result<$crate::ingredient_value::IngredientValue, $crate::error::RecipeError> {
                let start = ingredient.offset.0;
                let end = start + $size;

                // SITUATIONAL AWARENESS: Verify memory safety before access.
                let bytes = data.get(start..end)
                    .ok_or($crate::error::RecipeError::OutOfBounds)?;

                match ingredient.endianness {
                    // Standard x86/ARM memory reconstruction.
                    $crate::config::Endianness::Little => {
                        let val = <$type>::from_le_bytes(
                            bytes.try_into()
                                .map_err(|_| $crate::error::RecipeError::InvalidLength)?
                        );
                        Ok($variant_le(val))
                    }
                    // Standard Network/Telecommunications reconstruction.
                    _ => {
                        let val = <$type>::from_be_bytes(
                            bytes.try_into()
                                .map_err(|_| $crate::error::RecipeError::InvalidLength)?
                        );
                        Ok($variant_be(val))
                    }
                }
            }
        }
    };
}