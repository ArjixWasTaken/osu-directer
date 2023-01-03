#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Configuration {
    #[serde(with = "serde_path")]
    pub browser_path: Option<PathBuf>,
    #[serde(with = "serde_path")]
    pub custom_osu_path: Option<PathBuf>,
}

impl Configuration {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Configuration> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader)?;

        Ok(configuration)
    }

    pub fn write_default<P: AsRef<Path>>(path: P) -> std::io::Result<Configuration> {
        let config = Configuration::default();
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &config)?;

        Ok(config)
    }
}

mod serde_path {
    use std::path::PathBuf;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<PathBuf>, D::Error> {
        let path = <Option<String> as Deserialize>::deserialize(d)?;

        if let Some("auto" | "") = path.as_deref() {
            Ok(None)
        } else {
            Ok(path.map(PathBuf::from))
        }
    }

    pub fn serialize<S: Serializer>(path: &Option<PathBuf>, s: S) -> Result<S::Ok, S::Error> {
        match path {
            Some(path) => s.serialize_some(path),
            None => s.serialize_str("auto"),
        }
    }
}
