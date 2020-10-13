use std::path::Path;

use crate::consts::{CHATWHEEL_SOCKET_PATH, HEIGHT, NAME, WIDTH};

pub fn is_initialized() -> bool {
    Path::new(CHATWHEEL_SOCKET_PATH).exists()
}

pub fn initialize() {
    // TODO: https://unix.stackexchange.com/a/594698
    //
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

    use std::process::Command;

    let micsource = Command::new("sh")
        .args(&[
            "-c",
            "pactl list sources short | grep alsa_input | awk '{print$2}'",
        ])
        .output()
        .unwrap()
        .stdout;
    let micsource = String::from_utf8(micsource).unwrap();

    let script = vec![
        // create a unix pipe for writing data
        format!("pactl load-module module-pipe-source source_name=Chatwheel file={}", CHATWHEEL_SOCKET_PATH),
        // create a null-sink for merging microphone input and pipe
        format!("pactl load-module module-null-sink sink_name=ChatwheelMicSink sink_properties=device.description=ChatwheelMicSink"),
        // connect pipe with mixing sink
        format!("pactl load-module module-loopback sink=ChatwheelMicSink source=Chatwheel latency_msec={}", 100),
        // connect microphone input with mixing sink
        format!("pactl load-module module-loopback sink=ChatwheelMicSink source={}", micsource),

        //format!("pactl load-module module-combine-sink sink_name=ChatwheelMixer sink_properties=device.description=ChatwheelMixer"),
        //format!("pactl load-module module-null-sink sink_name=ChatwheelMicSink sink_properties=device.description=ChatwheelMicSink"),
        //format!("pactl load-module module-combine-sink sink_name=ChatwheelMixer sink_properties=device.description=ChatwheelMixer slaves=ChatwheelSink,ChatwheelMicSink"),
        //format!("pactl load-module module-loopback sink=ChatwheelMixer source={}", micsink),
        //"pactl load-module module-loopback sink=ChatwheelMixer source=Chatwheel".to_string(),
        //"pactl load-module module-combine-sink sink_name=ChatwheelMixer, slaves=ChatwheelSink,@DEFAULT_SOURCE@".to_string(),
        //"pactl load-module module-echo-cancel source_name=ChatwheelMic source_master=ChatwheelMixer.monitor aec_method=null source_properties=device.description=ChatwheelMic sink_properties=device.description=ChatwheelMic".to_string(),
    ];

    let mut loaded_modules = vec![];

    for (i, script_line) in script.iter().enumerate() {
        println!("{}: {}", i + 1, script_line);

        let output = Command::new("sh")
            .args(&["-c", script_line])
            .output()
            .expect("failed to spawn process");
        let stdout = String::from_utf8(output.stdout).unwrap();

        if !output.status.success() {
            println!(
                "command {} failed: {}",
                i + 1,
                String::from_utf8(output.stderr).unwrap()
            );
            break;
        }

        loaded_modules.push(stdout.trim().parse::<u32>().unwrap());
    }

    /*
    for mid in loaded_modules.iter() {
        Command::new("sh").args(&["-c", &format!("pactl unload-module {}", mid)]).output().unwrap();
    }
    */
}
