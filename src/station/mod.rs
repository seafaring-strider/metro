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
    // Telemetry for specific service
    telemetry: BTreeMap<String, bool>,
}

impl Station {
    /// Creates a new station from a vector of telemetry Fields.
    pub fn new(fields: Vec<Field>) -> Self {
        let mut telemetry = BTreeMap::new();
        for field in fields {
            telemetry.insert(field.name, field.standard);
        }
        Station { telemetry }
    }

    /// Reports the standard telemetry packet as a vector of Strings.
    pub fn report_standard(&self) -> Vec<String> {
        let mut standard_fields = Vec::new();
        for (field, standard) in &self.telemetry {
            if *standard {
                standard_fields.push(field.clone());
            }
        }
        standard_fields
    }

    /// Reports all telemetry as a vector of Strings.
    pub fn report_all(&self) -> Vec<String> {
        let mut all_fields = Vec::new();
        for field in self.telemetry.keys() {
            all_fields.push(field.clone());
        }
        all_fields
    }
}
