use std::collections::HashMap;

use chrono::prelude::*;
use semver::Version;
use serde::Deserialize;
use ssri::Integrity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EntropicPackument {
    #[serde(default = "Utc::now")]
    pub created: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub modified: DateTime<Utc>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub require_tfa: bool,
    #[serde(default = "HashMap::new")]
    pub tags: HashMap<String, Version>,
    #[serde(default = "HashMap::new")]
    pub versions: HashMap<Version, Integrity>,
}

impl Default for EntropicPackument {
    fn default() -> Self {
        EntropicPackument {
            created: Utc::now(),
            modified: Utc::now(),
            name: "".into(),
            require_tfa: false,
            tags: HashMap::new(),
            versions: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    #[test]
    fn empty_object() {
        let json = "{}";
        let packument: EntropicPackument = serde_json::from_str(json).unwrap();
        assert_eq!(
            packument,
            EntropicPackument {
                created: packument.created.clone(),
                modified: packument.modified.clone(),
                ..EntropicPackument::default()
            }
        );
    }
}
