use std::{collections::BTreeMap, fmt};

use time::{OffsetDateTime, UtcOffset};

pub struct Measurement {
    id: String,
    pub timestamp: i128,
    tags: BTreeMap<String, String>,
    fields: BTreeMap<String, f64>,
}

impl Measurement {
    pub fn new(id: &str, timestamp: OffsetDateTime) -> Self {
        Measurement {
            id: id.to_owned(),
            timestamp: timestamp.to_offset(UtcOffset::UTC).unix_timestamp_nanos(),
            tags: BTreeMap::new(),
            fields: BTreeMap::new(),
        }
    }

    pub fn add_tag(&mut self, key: &str, value: &str) {
        self.tags.insert(key.to_owned(), value.to_owned());
    }

    pub fn add_field(&mut self, key: &str, value: f64) {
        assert!(value.is_finite());

        self.fields.insert(key.to_owned(), value);
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(!self.fields.is_empty());

        let tags = self
            .tags
            .iter()
            .map(|(k, v)| format!("{}={}", escape(k), escape(v)))
            .collect::<Vec<String>>();

        let fields = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}={}", escape(k), v))
            .collect::<Vec<String>>();

        if !tags.is_empty() {
            f.pad(&format!(
                "{},{} {} {}",
                self.id,
                tags.join(","),
                fields.join(","),
                self.timestamp
            ))
        } else {
            f.pad(&format!(
                "{} {} {}",
                self.id,
                fields.join(","),
                self.timestamp
            ))
        }
    }
}

fn escape(tag: &str) -> String {
    tag.replace(' ', "\\ ").replace(',', "\\,")
}
