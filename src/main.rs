use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

type Lines = HashMap<String, Line>;

const NAME: &str = "chatwheel-rs";
const CHATWHEEL_ALL_PATH: &str = "./data/chatwheel.json";
const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    text: String,
    audios: Vec<String>,
}

fn load() -> Result<Lines, Box<dyn Error>> {
    let file = File::open(CHATWHEEL_ALL_PATH)?;
    let reader = BufReader::new(file);
    let lines: Lines = serde_json::from_reader(reader)?;
    Ok(lines)
}

fn main() {

    for (key, value) in load().unwrap().iter() {
        println!("{:?}: {:?}", key, value);
    }

}
