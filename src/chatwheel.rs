use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::consts::{CHATWHEEL_CONF_AUDIO_PATH, CHATWHEEL_CONF_PATH, CHATWHEEL_DEFAULT, NAME};
use crate::line::{load, Line};

pub fn chatwheel_config_dir() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push(NAME);
    config_dir
}

fn fetch_audio(id: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url).unwrap();
    let bytes = response.bytes()?;

    let file_path = get_audio_file_path(id)?;
    let file = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes).unwrap();

    Ok(())
}

pub fn create_config_file<T, U>(path: T, ids: &[U]) -> Result<(), Box<dyn Error>>
where
    T: AsRef<std::path::Path>,
    U: ToString + AsRef<str>,
{
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
        create_config_file(path, CHATWHEEL_DEFAULT)?;
    }

    Ok(())
}

pub fn get_audio_file_path(id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut audio_dir = chatwheel_config_dir();
    audio_dir.push(CHATWHEEL_CONF_AUDIO_PATH);

    for audio in std::fs::read_dir(audio_dir).expect("cannot read audio dir") {
        let audio = audio?;
        if audio.file_name().to_str().unwrap().starts_with(id) {
            return Ok(audio.path());
        }
    }

    unreachable!()
}

pub fn get_audio_file(id: &str) -> Result<File, Box<dyn Error>> {
    let audio_file_path = get_audio_file_path(id)?;
    Ok(File::open(audio_file_path).unwrap())
}

pub struct Chatwheel {
    pub forward_audio_enabled: bool,
    pub lines: Vec<Line>,
    pub profile: Option<String>,
}

impl Chatwheel {
    pub fn empty() -> Self {
        Self {
            forward_audio_enabled: false,
            lines: vec![],
            profile: None,
        }
    }

    pub fn load_by_profile(id: &str) -> Result<Self, Box<dyn Error>> {
        let mut config_file = chatwheel_config_dir();
        config_file.push(format!("{}.json", id));
        let mut obj = Self::load(config_file)?;
        obj.set_profile(id.to_string());
        Ok(obj)
    }

    pub fn load<T>(config_file: T) -> Result<Self, Box<dyn Error>>
    where
        T: AsRef<std::path::Path>,
    {
        let mut obj = Self::empty();
        let file = File::open(config_file)?;
        let reader = BufReader::new(file);
        obj.lines = serde_json::from_reader(reader)?;
        Ok(obj)
    }

    pub fn set_profile(&mut self, profile: String) {
        self.profile = Some(profile);
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
