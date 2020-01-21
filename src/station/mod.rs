use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub standard: bool,
}

#[derive(Debug, PartialEq)]
pub struct Station {
    pub telemetry: BTreeMap<String, bool>,
}

impl Station {
    pub fn new(fields: Vec<Field>) -> Self {
        let mut telemetry = BTreeMap::new();
        for field in fields {
            telemetry.insert(field.name, field.standard);
        }
        Station {
            telemetry,
        }
    }

    pub fn report_standard(&self) -> Vec<String> {
        let mut standard_fields = Vec::new();
        for (field, standard) in &self.telemetry {
            if *standard {
                standard_fields.push(field.clone());
            }
        }
        standard_fields
    }

    pub fn report_all(&self) -> Vec<String> {
        let mut all_fields = Vec::new();
        for (field, standard) in &self.telemetry {
            all_fields.push(field.clone());
        }
        all_fields
    }
}