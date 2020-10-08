use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::consts::{CHATWHEEL_CONF_PATH, CHATWHEEL_DEFAULT, NAME};
use crate::line::{load, Line};

fn init_config_dir(mut path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        create_dir(path.clone())?;
    }

    path.push(CHATWHEEL_CONF_PATH);

    if !path.exists() {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let all_lines = load()?;
        let mut lines = vec![];

        for id in CHATWHEEL_DEFAULT.iter() {
            lines.push(all_lines.get(&id.to_string()).unwrap());
        }

        let json_str = serde_json::to_string(&lines)?;
        writer.write_all(json_str.as_bytes())?;
    }

    Ok(())
}

pub struct Settings {
    pub lines: Vec<Line>,
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push(NAME);

        init_config_dir(config_dir.clone())?;

        let mut config_file = config_dir.clone();
        config_file.push(CHATWHEEL_CONF_PATH);

        let file = File::open(config_file)?;
        let reader = BufReader::new(file);
        let lines: Vec<Line> = serde_json::from_reader(reader)?;
        Ok(Self { lines })
    }
}
