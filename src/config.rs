#![allow(dead_code)]

use webextension_pattern::Pattern;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub browser_path: String,
    pub custom_osu_path: Option<String>,
}

impl Configuration {
    pub fn empty() -> Self {
        Self {
            browser_path: "auto".into(),
            custom_osu_path: Some("".into()),
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
