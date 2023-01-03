#![allow(dead_code)]

use serde::{Deserialize, Deserializer, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(deserialize_with = "deserialize_path")]
    pub browser_path: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_path")]
    pub custom_osu_path: Option<PathBuf>,
}

impl Configuration {
    pub fn empty() -> Self {
        Self {
            browser_path: None,
            custom_osu_path: None,
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Configuration> {
        let file = File::open(path).map_err(|x| x)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader).map_err(|x| x)?;
        Ok(configuration)
    }

    pub fn write_default<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        std::fs::write(path, serde_json::to_string(&Configuration::empty())?)?;

        Ok(())
    }
}

fn deserialize_path<'de, D: Deserializer<'de>>(d: D) -> Result<Option<PathBuf>, D::Error> {
    let path = <Option<String> as Deserialize>::deserialize(d)?;

    if let Some("auto" | "") = path.as_deref() {
        Ok(None)
    } else {
        Ok(path.map(PathBuf::from))
    }
}
