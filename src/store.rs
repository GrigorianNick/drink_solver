use std::{fs::File, io::BufReader, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};

use crate::builder::Builder;

pub trait Store<T>: Serialize + DeserializeOwned + Default {

    fn get_json_name() -> PathBuf;

    fn set_config_path(&mut self, path: PathBuf);

    fn get_config_path(&self) -> Option<PathBuf>;

    fn register(&mut self, entry: T) -> uuid::Uuid;

    fn get_entries(&self) -> Vec<T>;

    fn get_entries_mut(&mut self) -> Vec<&mut T>;

    fn get_entry(&self, id: uuid::Uuid) -> Option<T>;

    fn get_entry_mut(&mut self, id: uuid::Uuid) -> Option<&mut T>;

    fn build_from<B: Builder<T>>(&mut self, builder: &B) -> uuid::Uuid {
        self.register(builder.build())
    }

    fn from_config(config_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let path = config_dir.join(Self::get_json_name());
        if std::fs::exists(&path)? {
            let cfg_file = File::open(&path)?;
            let cfg = BufReader::new(cfg_file);
            let mut store: Self = serde_json::from_reader(cfg)?;
            store.set_config_path(path);
            Ok(store)
        } else {
            std::fs::File::create_new(&path)?;
            let mut store = Self::default();
            store.set_config_path(path);
            Ok(store)
        }
    }

    fn new() -> Self {
        //let cfg_dir = dirs::config_local_dir().unwrap_or(PathBuf::from("."));
        let cfg_dir = PathBuf::from("./");
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