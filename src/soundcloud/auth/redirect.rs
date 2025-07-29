use std::{
    error::Error, fs,
    sync::{Arc, Mutex}
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}
};

use crate::app::{App, Mode};
use super::super::config::ClientConfig;

pub async fn serve(client_config: &mut ClientConfig, app: &Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                handle_connection(stream, client_config, app).await?;
                match client_config.client_code() {
                    Some(_) => break, 
                    None => continue
                }
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
    Ok(())
}

async fn handle_connection(
    mut stream: TcpStream,
    client_config: &mut ClientConfig,
    app: &Arc<Mutex<App>>,
    ) -> Result<(), Box<dyn Error>>{

    let mut buffer = [0; 512];
    stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    let (status_line, filename) = match &request_line[0..10] {
        "GET /?code" => {
            let code = fetch_client_code(request_line);
            client_config.set_client_code(code);
            app.lock().unwrap().mode = Mode::Normal;

            ("HTTP/1.1 200 OK", "src/static/redirect.html")
        },
        _ =>  ("HTTP/1.1 400 BAD REQUEST", "src/static/error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
        
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await?;
    
    Ok(())
}

fn fetch_client_code(request_line: &str) -> String {
    let code = request_line
        .split("code=")
        .last()
        .unwrap()
        .split("&")
        .next()
        .unwrap();

    code.to_string()
}
