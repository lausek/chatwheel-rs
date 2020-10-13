use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};

use crate::consts::{CHATHWHEEL_PIPE_PATH, HEIGHT, NAME, WIDTH};
use crate::pulseaudio;
use crate::settings::{get_audio_file, get_audio_file_path, Settings};

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

pub fn forward_audio(id: &str) {
    if !pulseaudio::is_initialized() {
        pulseaudio::initialize();
    }

    let line_path = get_audio_file_path(&id);

    std::process::Command::new("sh")
        .args(&[
            "-c",
            &format!(
                "ffmpeg -re -i {} -f s16le -ar 44100 -ac 2 - > {}",
                line_path.as_path().display(),
                CHATHWHEEL_PIPE_PATH,
            ),
        ])
        .spawn()
        .unwrap();

    /*
    // TODO: this is basically trash
    //let mut buffer = vec![];
    let line_file = get_audio_file(&id);
    let decoder = rodio::Decoder::new(line_file).unwrap();
    let mut socket = std::fs::OpenOptions::new()
                        .append(true)
                        .open(CHATHWHEEL_PIPE_PATH)
                        .unwrap();

    //let mut socket = unix_named_pipe::open_write(CHATHWHEEL_PIPE_PATH).unwrap();

    for sample in decoder {
        let hb = ((sample >> 8) & 0xF) as u8;
        let lb = (sample & 0xF) as u8;
        socket.write(&[hb, lb]).unwrap();
    }

    use std::io::Write;

    //socket.write_all(&buffer).unwrap();

    socket.flush().unwrap();
    */
}

pub fn play_audio_file(id: &str) {
    let id = id.to_string();
    let audio_file = get_audio_file(&id);
    let decoder = rodio::Decoder::new(audio_file).unwrap();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(decoder);
    sink.sleep_until_end();
}

// circular grid pattern
fn get_coords(_n: usize) -> &'static [(i32, i32)] {
    &[
        (1, 1),
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (2, 1),
        (2, 5),
        (3, 1),
        (3, 5),
        (4, 1),
        (4, 5),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
    ]
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

        let component = gtk::Grid::new();
        let coords = get_coords(self.settings.lines.len());

        for (line, (x, y)) in self.settings.lines.iter().zip(coords.iter()) {
            let button = gtk::Button::new_with_label(line.text.as_str());

            let button_line = line.clone();
            let forward_audio_enabled = self.settings.forward_audio_enabled;

            button.connect_clicked(move |_| {
                if !button_line.audios.is_empty() {
                    let id = button_line.id.as_ref().unwrap();

                    use std::process::Command;
                    Command::new("sh")
                        .args(&[
                            "-c",
                            format!(
                                "chatwheel-rs --play {} {}",
                                id,
                                if forward_audio_enabled {
                                    "--forward-to-mic"
                                } else {
                                    ""
                                }
                            )
                            .as_ref(),
                        ])
                        .spawn()
                        .expect("failed to spawn process");
                }

                close();
            });

            component.attach(&button, *x, *y, 1, 1);
        }

        component.set_column_spacing(2);
        component.set_row_spacing(2);
        component.show_all();

        window.add(&component);

        window.show_all();

        gtk::main();
    }
}
