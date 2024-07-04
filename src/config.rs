use crate::regions::Region;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    regions: BTreeMap<String, Region>,
}

impl Config {
    pub fn default() -> Self {
        let default_region = Region::default();
        Self {
            regions: BTreeMap::from([(default_region.name().into(), default_region)]),
        }
    }

    pub fn load() -> Result<Self, SerDeError> {
        let config_file = Config::get_config_file().unwrap();
        if !config_file.exists() {
            let instance = Self::default();
            let result = instance.save(config_file.as_ref());
            if result.is_err() {
                eprintln!(
                    "Failed to save the default config! {:#?}",
                    result.err().unwrap()
                );
            }
            return Ok(instance);
        }

        let mut reader = match std::fs::File::open(config_file) {
            Ok(file) => file,
            Err(err) => return Err(SerDeError::IoError(err)),
        };
        let mut data = vec![];
        let read_result = reader.read_to_end(&mut data);
        if read_result.is_err() {
            let err = read_result.err().unwrap();
            return Err(SerDeError::IoError(err));
        }

        let instance: Self = match serde_json::from_slice(data.as_slice()) {
            Ok(res) => res,
            Err(err) => return Err(SerDeError::SerdeJsonError(err)),
        };
        Ok(instance)
    }

    pub fn save(&self, path: &Path) -> Result<(), SerDeError> {
        let json = match serde_json::to_string_pretty(self) {
            Ok(val) => val,
            Err(err) => return Err(SerDeError::SerdeJsonError(err)),
        };
        let parent_dir = path.parent();
        if parent_dir.is_some() {
            match std::fs::create_dir_all(parent_dir.unwrap()) {
                Ok(_) => {}
                Err(err) => return Err(SerDeError::IoError(err)),
            }
        }
        let mut dest_file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(SerDeError::IoError(err)),
        };
        return match dest_file.write_all(json.as_ref()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SerDeError::IoError(err)),
        };
    }

    pub fn set_region(&mut self, region: Region) -> Result<(), SerDeError> {
        self.regions.insert(region.name().to_string(), region);
        self.save(Self::get_config_file().unwrap().as_path())
    }

    pub fn regions(&self) -> &BTreeMap<String, Region> {
        &self.regions
    }

    pub fn get_config_dir() -> Option<PathBuf> {
        let config_path = Path::new("raws");
        let user_config_dir = match dirs::config_dir() {
            Some(path_buf) => path_buf,
            None => return None,
        };
        Some(user_config_dir.join(config_path))
    }

    pub fn get_config_file() -> Option<PathBuf> {
        match Config::get_config_dir() {
            Some(dir) => Some(dir.join("config.json")),
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum SerDeError {
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
}
