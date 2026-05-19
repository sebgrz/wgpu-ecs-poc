use std::{collections::HashMap, fs::File, io::Read, ops::Not};

pub type Error = String;

pub enum AssetType {
    Texture {
        path: String,
        width: i32,
        height: i32,
    },
    Shader {
        shader_str: String,
    }, // TODO: music, level, so eon
}

pub struct AssetManager {
    assets_map: HashMap<String, AssetType>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, asset_id: &str, asset: AssetType) -> Result<(), Error> {
        if self.assets_map.contains_key(asset_id) {
            return Err("asset_id exists".to_string());
        }

        self.assets_map.insert(asset_id.to_owned(), asset);

        Ok(())
    }

    pub fn load_bytes(&self, asset_id: &str) -> Result<Vec<u8>, Error> {
        if self.assets_map.contains_key(asset_id).not() {
            return Err("asset_id is not exists".to_string());
        }
        let asset = self
            .assets_map
            .get(asset_id)
            .ok_or(format!("asset_id {} doesn't exists", asset_id))?;
        match asset {
            AssetType::Texture { path, .. } => {
                let mut file = File::open(path).unwrap();
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();
                return Ok(bytes);
            }
            _ => Err("asset type is not supported".to_string()),
        }
    }
}
