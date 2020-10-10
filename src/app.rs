use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};
use std::fs::File;

use crate::consts::{CHATWHEEL_SOCKET_PATH, HEIGHT, NAME, WIDTH};
use crate::settings::{get_audio_file, Settings};

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

pub fn forward_audio(id: &str) {
    // TODO: https://unix.stackexchange.com/a/594698
    //
    // # create a unix pipe for writing data
    // pactl load-module module-pipe-source source_name="Chatwheel" file="/tmp/chatwheel-rs.input"
    // # create a null-sink for transforming pipe into a sink
    // pactl load-module module-null-sink sink_name="ChatwheelSink" source_name="Chatwheel"
    //
    // # create a mixing sink
    // pactl load-module module-combine-sink sink_name="ChatwheelMixer"
    // slaves=ChatwheelSink,<mix_source>
    //
    // # Discord does not accept the Sink.monitor, so we turn it into a new device using echo
    // # cancellation (original code had sink_master=silence as argument, but this didn't work)
    // pactl load-module module-echo-cancel sink_name=ChatwheelMic source_name=ChatwheelMic source_master=ChatwheelMixer.monitor aec_method=null source_properties=device.description=ChatwheelMic \ sink_properties=device.description=ChatwheelMic
    //
    // ffmpeg -re -i ~/.config/chatwheel-rs/audio/ancient6.mp3.mpeg.mpga -f s16le -ar 44100 -ac 2 - > /tmp/chatwheel-rs.input

    let mut buffer = vec![];
    let mut socket = File::create(CHATWHEEL_SOCKET_PATH).unwrap();
    let mut line = get_audio_file(&id);

    use std::io::Read;
    line.read_to_end(&mut buffer).unwrap();
    use std::io::Write;
    socket.write_all(&buffer).unwrap();

    socket.flush().unwrap();
}

pub fn play_audio_file(id: &str) {
    use rodio::Source;
    use std::thread;

    let id = id.to_string();
    let audio_file = get_audio_file(&id);
    let decoder = rodio::Decoder::new(audio_file).unwrap();

    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(decoder);
    sink.sleep_until_end();
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
                if !button_line.audios.is_empty() {
                    let id = button_line.id.as_ref().unwrap();

                    use std::process::Command;
                    Command::new("sh")
                        .args(&["-c", format!("chatwheel-rs --play {}", id).as_ref()])
                        .spawn()
                        .expect("failed to spawn process");
                }

                close();
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
