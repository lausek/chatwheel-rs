use std::path::Path;

use crate::consts::CHATHWHEEL_PIPE_PATH;

pub fn is_initialized() -> bool {
    Path::new(CHATHWHEEL_PIPE_PATH).exists()
}

pub fn initialize() {
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
        format!("pactl load-module module-pipe-source source_name=Chatwheel file={}", CHATHWHEEL_PIPE_PATH),
        // create a null-sink for merging microphone input and pipe
        "pactl load-module module-null-sink sink_name=ChatwheelMicSink sink_properties=device.description=ChatwheelMicSink".to_string(),
        // connect pipe with mixing sink
        format!("pactl load-module module-loopback sink=ChatwheelMicSink source=Chatwheel latency_msec={}", 100),
        // connect microphone input with mixing sink
        format!("pactl load-module module-loopback sink=ChatwheelMicSink source={}", micsource),

        "pactl load-module module-null-sink sink_name=drop".to_string(),
        "pactl load-module module-echo-cancel source_name=ChatwheelMic source_master=ChatwheelMicSink.monitor sink_master=drop aec_method=null source_properties=device.description=ChatwheelMic sink_properties=device.description=ChatwheelMic".to_string(),
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
