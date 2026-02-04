use crate::{
    config::{Body, Config, Header, Entry},
    types::{Length, MagicNumber, Offset},
    error::RecipeError,
    ingredient::Ingredient,
    ingredient_builder::IngredientBuilder,
};

/// The Orchestrator for Protocol Schema Construction.
///
/// `RecipeBuilder` manages the stateful assembly of a `Config`. It utilizes a 
/// "Stage-Gate" approach, where a `Header` and `Body` are defined incrementally, 
/// ensuring that complex nested structures (like repeating entries) are 
/// contextually valid before finalization.
pub struct RecipeBuilder {
    header: Header,
    body: Body,
    /// A temporary staging area for the entry currently being configured.
    /// This prevents "Partial Entry" leakage into the final production config.
    current_entry: Option<Entry>,
}

impl RecipeBuilder {
    /// Initializes a blank protocol template.
    ///
    /// Sets up default spatial jurisdictions (Offset 0, Length 0) and 
    /// prepares the entry vector for population.
    pub fn new() -> Self {
        RecipeBuilder {
            header: Header {
                magic_number: MagicNumber(String::new()),
                offset: Offset(0),
                length: Length(0),
            },
            body: Body {
                timestamp: IngredientBuilder::new().build(),
                entries: Vec::new(),
            },
            current_entry: None,
        }
    }

    /// Configures the Protocol Identity (The "Magic" header).
    ///
    /// This defines the 'Signature' of the file/stream and the spatial 
    /// footprint of the metadata preamble.
    pub fn header(mut self, magic_number: &str, offset: usize, length: usize) -> Self {
        self.header = Header {
            magic_number: MagicNumber(magic_number.to_string()),
            offset: Offset(offset),
            length: Length(length),
        };
        self
    }

    /// Attaches a global ingredient (like a File-Level Timestamp) to the body.
    pub fn add_ingredient_to_body(mut self, ingredient: Ingredient) -> Self {
        self.body.timestamp = ingredient;
        self
    }

    /// STAGING GATE: Initiates a new repeating data unit.
    ///
    /// In a 500k LoC system, packets often contain arrays of records. 
    /// This method creates a "Sandbox" (the `current_entry`) where specific 
    /// ingredients can be added before being committed to the main Body.
    pub fn new_entry(mut self, offset: usize) -> Result<Self, RecipeError> {
        self.current_entry = Some(Entry {
            offset: Offset(offset),
            value: IngredientBuilder::new().build(),
            timestamp: IngredientBuilder::new().build(),
        });
        Ok(self)
    }

    /// Maps a payload ingredient to the entry currently in the staging area.
    pub fn ingredient_to_entry(mut self, ingredient: Ingredient) -> Result<Self, RecipeError> {
        if let Some(entry) = &mut self.current_entry {
            entry.value = ingredient;
            Ok(self)
        } else {
            Err(RecipeError::InvalidConfig("No current entry to add ingredient to".to_string()))
        }
    }

    /// COMMIT POINT: Moves the staged entry into the permanent Body collection.
    ///
    /// This acts as a transactional boundary, ensuring that only "Started" 
    /// entries can be pushed into the final configuration.
    pub fn add_entry(mut self) -> Result<Self, RecipeError> {
        if let Some(entry) = self.current_entry.take() {
            // Note: Deep validation is deferred until the final .build() call.
            self.body.entries.push(entry);
            Ok(self)
        } else {
            Err(RecipeError::InvalidConfig("No current entry to add".to_string()))
        }
    }

    /// Bulk-configures the body content.
    pub fn body(mut self, timestamp: Ingredient, entries: Vec<Entry>) -> Self {
        self.body = Body { timestamp, entries };
        self
    }
    
    /// GLOBAL INTEGRITY CHECK:
    /// Validates the entire blueprint for spatial and logical sanity.
    ///
    /// ### Validation Rigor:
    /// - **Identity**: Ensures the Magic Number is present.
    /// - **Spatial Floor**: Ensures all critical components have a 
    ///   non-zero physical footprint (Length > 0).
    /// - **Deep Inspection**: Recursively checks every entry in the body
    ///   to prevent "Dead Zones" in the protocol lens.
    pub fn validate(&self) -> Result<(), RecipeError> {
        if self.header.magic_number.0.is_empty() {
            return Err(RecipeError::InvalidConfig("Magic number cannot be empty".to_string()));
        }
        if self.header.length.0 == 0 {
            return Err(RecipeError::InvalidConfig("Header length must be greater than zero".to_string()));
        }
        if self.body.timestamp.length.0 == 0 {
            return Err(RecipeError::InvalidConfig("Timestamp length must be greater than zero".to_string()));
        }
        for entry in &self.body.entries {
            if entry.timestamp.length.0 == 0 {
                return Err(RecipeError::InvalidConfig("Entry timestamp length must be greater than zero".to_string()));
            }
            if entry.value.length.0 == 0 {
                return Err(RecipeError::InvalidConfig("Entry value length must be greater than zero".to_string()));
            }
        }
        Ok(())
    }
    
    /// Maps a temporal ingredient to the entry currently in the staging area.
    pub fn timestamp_to_entry(mut self, timestamp: Ingredient) -> Result<Self, RecipeError> {
        if let Some(entry) = &mut self.current_entry {
            entry.timestamp = timestamp;
            Ok(self)
        } else {
            Err(RecipeError::InvalidConfig("No current entry to add timestamp to".to_string()))
        }
    }

    /// Finalizes the Blueprint.
    ///
    /// Consumes the builder, triggers a global `validate()`, and returns a 
    /// ready-to-use `Config`. This is the final step before the schema 
    /// is handed to the `ParserRegistry` for execution.
    pub fn build(self) -> Result<Config, RecipeError> {
        self.validate()?;
        Ok(Config {
            header: self.header,
            body: self.body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RecipeBuilder;
    use crate::{
        error::RecipeError,
        ingredient_builder::IngredientBuilder
    };

    #[test]
    fn test_valid_config_build() {
        let timestamp = IngredientBuilder::new()
            .name("timestamp")
            .offset_bytes(0)
            .length_bytes(4)
            .format("u8")
            .build();

        let value = IngredientBuilder::new()
            .name("value")
            .offset_bytes(4)
            .length_bytes(4)
            .format("u8")
            .build();

        let config = RecipeBuilder::new()
            .header("MAGIC", 0, 128)
            .add_ingredient_to_body(timestamp.clone())
            .new_entry(0)
            .unwrap()
            .ingredient_to_entry(value.clone())
            .unwrap()
            .add_entry()
            .unwrap()
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.header.magic_number.0, "MAGIC");
        assert_eq!(config.header.length.0, 128);
        assert_eq!(config.body.timestamp.name, "timestamp");
        assert_eq!(config.body.entries.len(), 1);
        assert_eq!(config.body.entries[0].value.name, "value");
    }

    #[test]
    fn test_missing_header_magic_number() {
        let timestamp = IngredientBuilder::new()
            .name("timestamp")
            .offset_bytes(0)
            .length_bytes(4)
            .format("u8")
            .build();

        let builder = RecipeBuilder::new()
            .header("", 0, 128)
            .add_ingredient_to_body(timestamp);

        let result = builder.build();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, RecipeError::InvalidConfig(_)));
    }

    #[test]
    fn test_missing_entry_value() {
        let timestamp = IngredientBuilder::new()
            .name("timestamp")
            .offset_bytes(0)
            .length_bytes(4)
            .format("u8")
            .build();

        let entry_timestamp = IngredientBuilder::new()
            .name("entry_timestamp")
            .offset_bytes(8)
            .length_bytes(4)
            .format("u8")
            .build();

        let builder = RecipeBuilder::new()
            .header("MAGIC", 0, 128)
            .add_ingredient_to_body(timestamp)
            .new_entry(0)
            .unwrap()
            .timestamp_to_entry(entry_timestamp) // ✅ otherwise it fails on length == 0
            .unwrap();

        // let result = builder.add_entry();
        // assert!(result.is_err()); // Still valid because we're missing value


        // Missing value set for entry
        let result = builder.add_entry();
        assert!(result.is_err());
    }

}