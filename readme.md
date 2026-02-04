# 📜 RECIPE: High-Assurance Protocol Awareness & Memory Lensing

**RECIPE** is an industrial-grade Rust framework designed to transform raw bitstreams into structured domain models with mathematical certainty. Built for environments where data integrity and thread-safety are non-negotiable, it utilizes a "Lensing" metaphor to resolve memory jurisdictions across heterogeneous systems.

## 🏗️ Core Architecture

The system is built on a "Defensive-in-Depth" stack, separating the physical layout of data from its logical interpretation.

### 1. The Blueprint (Ingredients)
Every data segment is defined as an `Ingredient`. It contains the **Spatial Jurisdiction** (Offset and Length) and the **Interpretive Policy** (Format and Endianness).
* **Recursive**: Ingredients can contain children, allowing for complex nested Structs and Arrays.
* **Defensive**: Integrated `max_length` guards prevent "Greedy Parsing" and DoS vulnerabilities.

### 2. The Engine (ParserRegistry)
The `ParserRegistry` acts as the Central Nervous System. It maps semantic format strings (e.g., `"u32"`, `"ipv4"`) to thread-safe, heap-allocated `ParserTrait` objects.
* **Feature-Gated**: Only compile the parsers you need (Signed, Unsigned, Network, Float).
* **Dynamic Dispatch**: Uses `Box<dyn ParserTrait>` for runtime selection of logic based on the schema blueprint.

### 3. The Fabric (Macros)
RECIPE utilizes a sophisticated macro-factory system:
* `impl_numeric_parser!`: Churns out scalar parsers with zero-cost abstraction and endian-awareness.
* `parse_bytes_array!`: Enforces fixed-width stack allocation and zero-panic boundary checks.
* `require_length!`: Acts as a physical gatekeeper to ensure data streams match expected signatures.

---

## 🚀 Quick Start: Building a Lens

```rust
// 1. Define your Ingredients using the Fluent Builder
let timestamp = IngredientBuilder::new()
    .name("body.timestamp")
    .offset_bytes(0)
    .length_bytes(8)
    .format("u64")
    .little_endian()
    .build();

// 2. Register your capabilities via the feature-gated Registry
let registry = ParserRegistry::new();

// 3. Execute the Lens against raw data
let raw_data = [0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
let value = registry.parse_ingredient(&raw_data, &timestamp)?;

assert_eq!(value.as_u64(), Some(&42));
```

## 🛡️ Safety & Security Invariants

| Feature | Implementation | Benefit |
| :--- | :--- | :--- |
| **OOB Protection** | `start_end_from_ingredient!` | Prevents buffer over-reads via strict `.get()` slicing and Result returns. |
| **Type Safety** | `IngredientValue` Enum | Eliminates "Boolean Blindness" and ensures type-safe data access via generated accessors. |
| **Endian Neutrality** | `parse_endian!` | Decouples data interpretation from Host CPU architecture (LE vs BE). |
| **Resilience** | `RecipeError` Domain | Converges on a deterministic error model for precise system forensics. |

---

## 🛠️ Feature Flags

RECIPE is highly modular. Enable only what your environment requires to minimize the binary footprint:

* **`signed` / `unsigned`**: Standard integer support (i8-i64, u8-u64).
* **`float`**: IEEE 754 support (f32, f64).
* **`network`**: `Ipv4`, `Ipv6`, and `MacAddress` support.
* **`composite`**: Support for `Struct` and `Array` recursive types.

---

## 🏗️ Development Guidelines

When extending the framework, follow the **Macro-First Principle**:

1. **New Primitives**: Use `impl_simple_parser!` or `impl_fixed_array_parser!` to ensure your new types inherit global safety checks.
2. **Accessors**: Always use `impl_accessor!` in `IngredientValue` to provide a non-consuming, reference-based API.
3. **Validation**: Every new `Ingredient` type must have a length check using the `require_length_for!` macro to maintain jurisdictional integrity.

> **Staff Engineer Note**: The `ParserRegistry` is designed to be a singleton or a long-lived resource. Because parsers are stateless and implement `Send + Sync`, the registry can be safely shared across your high-concurrency ingestion pipelines.