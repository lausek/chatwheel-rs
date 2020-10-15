use crate::chatwheel::{chatwheel_config_dir, create_config_file, Chatwheel};
use crate::consts::{CHATWHEEL_CONF_PATH, CONFIGURE_HOST, CONFIGURE_PORT, NAME};
use crate::line::load;
use crate::Settings;

fn configure_line_table(settings: &Settings) -> String {
    let chatwheel = if let Some(profile) = settings.profile.as_ref() {
        Chatwheel::load_by_profile(profile).unwrap_or(Chatwheel::empty())
    } else {
        Chatwheel::default()
    };
    let all_lines = load().unwrap();
    let mut all_lines = all_lines.into_iter().collect::<Vec<_>>();

    all_lines.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let lis = all_lines
        .iter()
        .map(|(line_id, line)| {
            let is_configured = chatwheel
                .lines
                .iter()
                .any(|line| line.id.as_ref().unwrap() == line_id);

            let audio_button = if let Some(audio) = line.audios.get(0) {
                format!(
                    "
                    <button type='button' onclick='this.nextElementSibling.play()'>Play</button>
                    <audio preload='none'><source src='{}' type='audio/mpeg'></audio>
                ",
                    audio
                )
            } else {
                format!("")
            };

            format!(
                "<tr>
                    <td class='line-checkbox'>
                        <input type='checkbox' id='{id}' name='{id}' {checked}>
                    </td>
                    <td class='line-id'>
                        {id}
                    </td>
                    <td class='line-audio'>
                        {audio}
                    </td>
                    <td class='line-text'>
                        <label for='{id}'>{line}</label>
                    </td>
                </tr>",
                id = line_id,
                line = line.text,
                checked = if is_configured { "checked" } else { "" },
                audio = audio_button,
            )
        })
        .collect::<Vec<_>>();

    format!("<table>{}</table>", lis.join("\n"))
}

fn configure_page(settings: &Settings) -> String {
    format!(
        "<html>
            <head>
                <title>chatwheel-rs config</title>
                <meta charset='utf-8'>
                <style>
                    #save-button {{
                        position: fixed;
                        top: 40px;
                        right: 40px;
                        font-size: 20px;
                    }}
                    tr:nth-child(odd) {{
                        background-color: #E6E6E6;
                    }}
                    td {{
                        padding: 3px;
                    }}
                </style>
            </head>
            <body>
            <form method='POST' action='/'>
                <button type='submit' id='save-button'>Save</button>
                {}
            </form>
            </body>
        </html>",
        configure_line_table(settings)
    )
}

pub fn run(settings: Settings) {
    let handler = move |request: &rouille::Request| match request.method() {
        "POST" => {
            let body = if let Some(mut reqbody) = request.data() {
                use std::io::Read;
                let mut body = String::new();
                reqbody.read_to_string(&mut body).unwrap();
                body
            } else {
                format!("")
            };

            let new_lines = body
                .split('&')
                .map(|kv| kv.split('=').collect::<Vec<_>>())
                .filter_map(|item| match item[..] {
                    [key, "on"] => Some(key),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let mut path = chatwheel_config_dir();
            path.push(NAME);
            if let Some(profile) = settings.profile.as_ref() {
                path.push(format!("{}.json", profile));
            } else {
                path.push(CHATWHEEL_CONF_PATH);
            }

            if create_config_file(path, &new_lines).is_ok() {
                std::process::exit(0);
            }

            rouille::Response::html(body)
        }
        _ => rouille::Response::html(configure_page(&settings)),
    };
    let server = rouille::Server::new((CONFIGURE_HOST, CONFIGURE_PORT), handler)
        .expect("could not create configure server");

    let hoststr = format!("http://{}:{}", CONFIGURE_HOST, CONFIGURE_PORT);

    std::process::Command::new("sh")
        .args(&[
            "-c",
            &format!("/etc/alternatives/x-www-browser {}", hoststr),
        ])
        .output()
        .unwrap();

    server.run();
}
