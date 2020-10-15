mod app;
mod chatwheel;
mod configure;
mod consts;
mod line;
mod pulseaudio;

use crate::app::{forward_audio, play_audio_file, App};
use crate::chatwheel::Chatwheel;

pub struct Settings {
    forward_to_mic: bool,
    play_id: Option<String>,
    profile: Option<String>,
}

impl From<clap::ArgMatches<'_>> for Settings {
    fn from(matches: clap::ArgMatches) -> Self {
        Self {
            forward_to_mic: matches.value_of("forward-to-mic").is_some(),
            play_id: matches
                .subcommand
                .as_ref()
                .and_then(|s| s.matches.value_of("id").map(str::to_string)),
            profile: matches.value_of("profile").map(str::to_string),
        }
    }
}

fn run_gui(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut chatwheel = if let Some(profile) = settings.profile {
        Chatwheel::load(profile)?
    } else {
        Chatwheel::default()
    };

    chatwheel.set_forward_audio(settings.forward_to_mic);

    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));
    App::new(chatwheel).run();

    Ok(())
}

fn run_config(settings: Settings) {
    configure::run(settings);
}

fn run_play(settings: Settings) {
    let play_id = settings.play_id.unwrap();

    if settings.forward_to_mic {
        forward_audio(play_id.as_ref());
    }

    play_audio_file(play_id.as_ref());
}

fn main() {
    use clap::{App, Arg, SubCommand};

    let args = App::new("chatwheel-rs")
        .arg(Arg::with_name("profile").long("profile").takes_value(true))
        .arg(Arg::with_name("forward-to-mic").long("forward-to-mic"))
        .subcommand(SubCommand::with_name("config"))
        .subcommand(
            SubCommand::with_name("play").arg(
                Arg::with_name("id")
                    .required(true)
                    .takes_value(true)
                    .value_name("ID"),
            ),
        )
        .get_matches();

    let subcommand_name = args.subcommand.as_ref().map(|s| s.name.clone());
    let settings = Settings::from(args);

    if let Some(name) = subcommand_name {
        match name.as_ref() {
            "config" => run_config(settings),
            "play" => run_play(settings),
            _ => panic!("unknown command {}", name),
        }
    } else {
        run_gui(settings).unwrap();
    }
}
