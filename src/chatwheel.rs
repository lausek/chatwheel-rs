use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::consts::{CHATWHEEL_CONF_AUDIO_PATH, CHATWHEEL_CONF_PATH, CHATWHEEL_DEFAULT, NAME};
use crate::line::{load, Line};

fn chatwheel_config_dir() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(NAME);
    config_dir
}

fn fetch_audio(id: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url).unwrap();
    let bytes = response.bytes()?;

    let file_path = get_audio_file_path(id);
    let file = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes).unwrap();

    Ok(())
}

pub fn create_config_file<T>(ids: &[T]) -> Result<(), Box<dyn Error>>
where
    T: ToString + AsRef<str>,
{
    let mut path = chatwheel_config_dir();
    path.push(CHATWHEEL_CONF_PATH);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let all_lines = load()?;
    let mut lines = vec![];

    for id in ids.iter() {
        let mut line_to_add = all_lines.get(&id.to_string()).unwrap().clone();
        line_to_add.id = Some(id.to_string());

        if !line_to_add.audios.is_empty() {
            let url = line_to_add.audios.get(0).unwrap();
            fetch_audio(id.as_ref(), url)?;
        }

        lines.push(line_to_add);
    }

    let json_str = serde_json::to_string(&lines)?;
    writer.write_all(json_str.as_bytes())?;

    Ok(())
}

fn init_config_dir(mut path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        create_dir(path.clone())?;

        let mut audio_dir = path.clone();
        audio_dir.push(CHATWHEEL_CONF_AUDIO_PATH);

        if !audio_dir.exists() {
            create_dir(audio_dir)?;
        }
    }

    path.push(CHATWHEEL_CONF_PATH);
    if !path.exists() {
        create_config_file(CHATWHEEL_DEFAULT)?;
    }

    Ok(())
}

pub fn get_audio_file_path(id: &str) -> PathBuf {
    let mut audio_file = chatwheel_config_dir();
    audio_file.push(CHATWHEEL_CONF_AUDIO_PATH);
    audio_file.push(format!("{}.mp3.mpeg.mpga", id));
    audio_file
}

pub fn get_audio_file(id: &str) -> File {
    let audio_file_path = get_audio_file_path(id);
    File::open(audio_file_path).unwrap()
}

pub struct Chatwheel {
    pub lines: Vec<Line>,
    pub forward_audio_enabled: bool,
}

impl Chatwheel {
    pub fn load<T>(config_file: T) -> Result<Self, Box<dyn Error>>
    where
        T: AsRef<std::path::Path>,
    {
        let file = File::open(config_file)?;
        let reader = BufReader::new(file);
        let lines: Vec<Line> = serde_json::from_reader(reader)?;

        Ok(Self {
            forward_audio_enabled: false,
            lines,
        })
    }

    pub fn set_forward_audio(&mut self, enabled: bool) {
        self.forward_audio_enabled = enabled;
    }
}

impl std::default::Default for Chatwheel {
    fn default() -> Self {
        let config_dir = chatwheel_config_dir();
        init_config_dir(config_dir.clone()).unwrap();

        let mut config_file = config_dir;
        config_file.push(CHATWHEEL_CONF_PATH);
        Self::load(config_file).unwrap()
    }
}
