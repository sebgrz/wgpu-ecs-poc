use std::{
    fs,
    sync::{Arc, RwLock},
};

use crate::data::{game_data::GameData, level::LevelData};

pub type ShareFileManager = Arc<RwLock<FileManager>>;

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        Self
    }

    pub fn load_game_data(&self) -> GameData {
        let raw_data = fs::read_to_string("res/game_data.ron").unwrap();
        let game_data: GameData = ron::from_str(&raw_data).unwrap();
        game_data
    }

    pub fn load_level(&self, number: i32) -> LevelData {
        let raw_data = fs::read_to_string(format!("res/levels/level_{}.ron", number)).unwrap();
        ron::from_str(&raw_data).unwrap()
    }
}
