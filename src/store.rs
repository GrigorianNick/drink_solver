use std::{fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::recipie_store::RecipieStore;

pub trait Store<T>: Serialize + DeserializeOwned + Default {

    fn get_json_name() -> PathBuf;

    fn set_config_path(&mut self, path: PathBuf);

    fn get_config_path(&self) -> Option<PathBuf>;

    fn register(&mut self, entry: T) -> uuid::Uuid;

    fn from_config(config_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let path = config_dir.join(Self::get_json_name());
        if std::fs::exists(&path)? {
            let cfg_file = File::open(path)?;
            let cfg = BufReader::new(cfg_file);
            let store = serde_json::from_reader(cfg)?;
            Ok(store)
        } else {
            std::fs::File::create_new(&path)?;
            let mut store = Self::default();
            store.set_config_path(path);
            Ok(store)
        }
    }

    fn new() -> Self {
        let cfg_dir = dirs::config_local_dir().unwrap_or(PathBuf::from("."));
        match Self::from_config(cfg_dir)
        {
            Ok(store) => store,
            Err(_) => Self::default()
        }
    }

    fn save(&self) -> bool {
        match self.get_config_path() {
            Some(path) => self.save_to(&path),
            None => false
        }
    }

    fn save_to(&self, path: &PathBuf) -> bool {
        match File::create(path) {
            Ok(file) => serde_json::to_writer(file, self).is_ok(),
            _ => false
        }
    }
}