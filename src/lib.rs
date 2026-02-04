//! # RECIPE
//! High-assurance composition and cryptographic manifest orchestration.
//! 
//! This crate provides the "Blueprint" layer for the Honest suite. It defines 
//! the exact procedures and ingredient lists required to assemble secure 
//! execution environments, ensuring that the combination of `classified` 
//! material and `allot`ed resources follows a deterministic, repeatable pattern.
//!
//! ## Core Security Principles
//! * **Repeatability:** Ensures that every secure environment is initialized 
//!   from a strictly defined, immutable recipe.
//! * **Composition:** Safely binds disparate primitives (logic, memory, IPC) 
//!   into a singular, verified workflow.
//! * **Declarative Security:** Defines the "Desired State" of the system, 
//!   allowing `assurance` to perform pre-flight audits.


pub mod error;
#[macro_use]pub mod macros;
pub mod traits;
pub mod utils;


pub mod config;

pub mod recipe_builder;


pub mod ingredient;
pub mod ingredient_builder;
pub mod ingredient_value;

pub mod parser_registry;

pub mod parsers;

pub mod safe_slice;


#[cfg(feature = "unsigned")]
pub mod payload;

pub mod types;