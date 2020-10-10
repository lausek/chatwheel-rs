use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};
use std::env::args;

mod app;
mod consts;
mod line;
mod settings;

use crate::app::{forward_audio, play_audio_file, App};
use crate::settings::Settings;

fn run_gui() {
    let settings = Settings::load().unwrap();
    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));
    App::new(settings).run();
}

fn run_config() {}

fn run_play(id: &str, forward_audio_enabled: bool) {
    if forward_audio_enabled {
        forward_audio(id);
    }

    play_audio_file(id);
}

fn main() {
    let mut args_iter = args();
    let _ = args_iter.next().unwrap();

    if let Some(cmd) = args_iter.next().as_ref() {
        match cmd.as_ref() {
            "config" => run_config(),
            "play" => {
                let id = args_iter.next().expect("no sound file id given");
                let forward_audio = true;
                run_play(&id, forward_audio);
            }
            _ => println!("command `{}` not known", cmd),
        }
    } else {
        run_gui()
    }
}
