use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};

mod app;
mod consts;
mod line;
mod settings;

use crate::app::App;
use crate::settings::Settings;

fn main() {
    /*
    for (key, value) in line::load().unwrap().iter() {
        println!("{:?}: {:?}", key, value);
    }
    */

    let settings = Settings::load().unwrap();
    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));

    App::new(settings).run();
}
