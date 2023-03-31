use serde::*;
use std::{fs::{File}, io::{Write, Read}};

pub fn write_settings(settings:Setting) {
    let serialized = serde_json::to_string(&settings).unwrap();
    let mut file = File::create("settings.json").unwrap();
    file.write(serialized.as_bytes()).unwrap();
}
pub fn read_settings() -> Setting {
    let mut settings = match File::open("settings.json") {
        Ok(fs) => fs,
        Err(_) => {
            write_settings(Setting::default());
            File::open("settings.json").unwrap()
        }
    };
    let mut contents = String::new();
    settings.read_to_string(&mut contents).unwrap();
    match serde_json::from_str::<Setting>(&contents) {
        Ok(o) => o,
        Err(_) => {
            write_settings(Setting::default());
            Setting::default()
        }
    }
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Setting {
    pub delay:u64,
    pub quality:u8,
}
impl Default for Setting {
    fn default() -> Self {
        Self {
            delay:20,
            quality:10
        }
    }
}
