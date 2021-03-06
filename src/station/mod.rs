/// Errors that can occur during Station operations.
pub mod errors;

use crate::station::errors::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
/// The definition of a telemetry field.
pub struct Field {
    /// Telemetry name
    pub name: String,
    /// Member of standard telemetry
    pub standard: bool,
}

#[derive(Debug, PartialEq)]
/// Stores, packages, and sends telemetry packets to other stations.
pub struct Station {
    // Structure of telemetry
    structure: BTreeMap<String, bool>,
    // Current telemetry values
    telemetry: BTreeMap<String, u32>,
}

impl Station {
    /// Creates a new station from a vector of telemetry Fields.
    pub fn new(fields: Vec<Field>) -> Self {
        let mut structure = BTreeMap::new();
        let mut telemetry = BTreeMap::new();
        for field in fields {
            structure.insert(field.name.clone(), field.standard);
            telemetry.insert(field.name, 0);
        }
        Station {
            structure,
            telemetry,
        }
    }

    /// Reports the standard telemetry packet as a vector of Strings.
    pub fn report_standard(&self) -> Vec<String> {
        let mut standard_fields = Vec::new();
        for (field, standard) in &self.structure {
            if *standard {
                standard_fields.push(field.clone());
            }
        }
        standard_fields
    }

    /// Reports all telemetry as a vector of Strings.
    pub fn report_all(&self) -> Vec<String> {
        let mut all_fields = Vec::new();
        for field in self.structure.keys() {
            all_fields.push(field.clone());
        }
        all_fields
    }

    /// Allows the reporting of a field's current value.
    pub fn get_field(&self, field: &str) -> StationResult<u32> {
        match self.telemetry.get(field) {
            Some(val) => Ok(*val),
            None => Err(StationError::TelemetryFieldMissing),
        }
    }

    /// Allows the setting of a field's current value.
    pub fn set_field(&mut self, field: &str, val: u32) -> StationResult<()> {
        if self.telemetry.contains_key(field) {
            self.telemetry.insert(field.to_string(), val);
            Ok(())
        } else {
            Err(StationError::TelemetryFieldMissing)
        }
    }
}
