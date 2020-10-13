use crate::consts::{CONFIGURE_HOST, CONFIGURE_PORT};
use crate::line::load;
use crate::settings::{create_config_file, Settings};

fn configure_line_table() -> String {
    let settings = Settings::load().unwrap();
    let all_lines = load().unwrap();
    let mut lis = vec![];

    for (line_id, line) in all_lines.iter() {
        let is_configured = settings
            .lines
            .iter()
            .any(|line| line.id.as_ref().unwrap() == line_id);

        let audio_button = if let Some(audio) = line.audios.get(0) {
            format!("<audio><source src='{}' type='audio/mpeg'></audio>", audio)
        } else {
            format!("")
        };

        let li_html = format!(
            "<li>
                <input type='checkbox' id='{id}' name='{id}' {checked}>
                <label for='{id}'>{line}</label>
                {audio}
            </li>",
            id = line_id,
            line = line.text,
            checked = if is_configured { "checked" } else { "" },
            audio = audio_button,
        );
        lis.push(li_html);
    }

    format!("<table><ul>{}</ul></table>", lis.join("\n"))
}

fn configure_page() -> String {
    format!(
        "<html>
            <head>
                <meta charset='utf-8'>
            </head>
            <body>
            <form method='POST' action='/'>
                {}
                <button type='submit'>Save</button>
            </form>
            </body>
        </html>",
        configure_line_table()
    )
}

pub fn run() {
    let handler = |request: &rouille::Request| match request.method() {
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
                .split("&")
                .map(|kv| kv.split("=").collect::<Vec<_>>())
                .filter_map(|item| match &item[..] {
                    &[key, "on"] => Some(key),
                    _ => panic!("invalid data given"),
                })
                .collect::<Vec<_>>();

            if create_config_file(&new_lines).is_ok() {
                std::process::exit(0);
            }

            rouille::Response::html(body)
        }
        _ => rouille::Response::html(configure_page()),
    };
    let server = rouille::Server::new((CONFIGURE_HOST, CONFIGURE_PORT), handler)
        .expect("could not create configure server");

    let hoststr = format!("http://{}:{}", CONFIGURE_HOST, CONFIGURE_PORT);

    std::process::Command::new("sh")
        .args(&["-c", &format!("firefox {}", hoststr)])
        .output()
        .unwrap();

    server.run();
}
