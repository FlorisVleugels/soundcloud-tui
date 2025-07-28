use std::{fs, 
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, sync::{Arc, Mutex}
};

use crate::app::{App, Mode};
use super::super::config::ClientConfig;

pub fn serve(client_config: &mut ClientConfig, app: &Arc<Mutex<App>>) {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, client_config, app);
        match client_config.client_code() {
            Some(_) => break, 
            None => continue
        }
    }
}

fn handle_connection(
    mut stream: TcpStream,
    client_config: &mut ClientConfig,
    app: &Arc<Mutex<App>>,
    ) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, filename) = match &request_line[0..10] {
        "GET /?code" => {
            let code = fetch_client_code(&request_line);
            client_config.set_client_code(code);
            app.lock().unwrap().mode = Mode::Normal;

            ("HTTP/1.1 200 OK", "src/static/redirect.html")
        },
        _ =>  ("HTTP/1.1 400 BAD REQUEST", "src/static/error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
        
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn fetch_client_code(request_line: &String) -> String {
    let code = request_line
        .split("code=")
        .last()
        .unwrap()
        .split("&")
        .next()
        .unwrap();

    code.to_string()
}
