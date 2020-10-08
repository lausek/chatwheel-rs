use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};

use crate::consts::{HEIGHT, NAME, WIDTH};
use crate::settings::{get_audio_file, Settings};

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

fn play_audio_file(id: &str) {
    let audio_file = get_audio_file(id);
}

pub struct App {
    settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> App {
        App { settings }
    }

    pub fn run(&self) {
        let provider = gtk::CssProvider::new();

        {
            /*
            let css_string = format!(
                "box, button {{
                                    border-radius:0;
                                    background-image:none;
                                    color:{0};
                                    border-color:{0};
                                    background-color:{1};
                                 }}
                                 label {{text-shadow:none}}
                                 window {{
                                    background-color:{0};
                                 }}
                                 box {{
                                    padding:15px;
                                 }}",
                f, b
            );
            provider.load_from_data(css_string.as_bytes()).ok();
            */
        }

        let window = Window::new(WindowType::Popup);
        window.set_keep_above(true);
        window.set_position(WindowPosition::Center);
        window.connect_delete_event(|_, _| close());
        window.set_title(NAME);

        let context = window.get_style_context();
        let screen = context.get_screen().unwrap();
        StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let component = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        for line in self.settings.lines.iter() {
            let button = gtk::Button::new_with_label(line.text.as_str());

            let button_line = line.clone();

            button.connect_clicked(move |_| {
                close();

                if !button_line.audios.is_empty() {
                    play_audio_file("ti10_174");
                    //get_audio_file("ti10_174");
                    //println!("{:?}", );
                }

                //std::process::exit(fcode);
            });

            component.pack_start(&button, true, true, 2);
        }

        component.set_spacing(2);
        component.show_all();

        window.add(&component);

        window.show_all();

        gtk::main();
    }
}
