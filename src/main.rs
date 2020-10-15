mod app;
mod chatwheel;
mod configure;
mod consts;
mod line;
mod pulseaudio;

use crate::app::{forward_audio, play_audio_file, App};
use crate::chatwheel::Chatwheel;

fn run_gui(forward_audio_enabled: bool) {
    let mut settings = Chatwheel::default();

    settings.forward_audio_enabled = forward_audio_enabled;

    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));
    App::new(settings).run();
}

fn run_config() {
    configure::run();
}

fn run_play(id: &str, forward_audio_enabled: bool) {
    if forward_audio_enabled {
        forward_audio(id);
    }

    play_audio_file(id);
}

fn main() {
    let mut play: Option<String> = None;
    let mut forward_audio_enabled = false;
    let mut config_requested = false;

    {
        use argparse::{ArgumentParser, StoreOption, StoreTrue};
        let mut parser = ArgumentParser::new();

        parser
            .refer(&mut play)
            .add_option(&["--play"], StoreOption, "output a chatwheel line");

        parser.refer(&mut forward_audio_enabled).add_option(
            &["--forward-to-mic"],
            StoreTrue,
            "forward chatwheel line as microphone input",
        );

        parser.refer(&mut config_requested).add_option(
            &["--config"],
            StoreTrue,
            "open the configuration",
        );

        parser.parse_args_or_exit();
    }

    if play.is_some() || config_requested {
        if let Some(ref id) = play {
            run_play(&id, forward_audio_enabled);
        }

        if config_requested {
            run_config()
        }
    } else {
        run_gui(forward_audio_enabled);
    }
}
