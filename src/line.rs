use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::consts::CHATWHEEL_ALL_PATH;

pub type Lines = HashMap<String, Line>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub id: Option<String>,
    pub text: String,
    #[serde(default)]
    pub audios: Vec<String>,
}

pub fn load() -> Result<Lines, Box<dyn Error>> {
    let file = File::open(CHATWHEEL_ALL_PATH)?;
    let reader = BufReader::new(file);
    let lines: Lines = serde_json::from_reader(reader)?;
    Ok(lines)
}
