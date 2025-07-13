use std::{fs, 
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}
};

pub fn serve() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

     for stream in listener.incoming() {
         let stream = stream.unwrap();

         handle_connection(stream);
     }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();


    let (status_line, filename) = match &request_line[0..10] {
        "GET /?code" => {
            fetch_client_code(&request_line);
            ("HTTP/1.1 200 OK", "src/static/redirect.html")
        },
        _ =>  ("HTTP/1.1 400 BAD REQUEST", "src/static/error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
        
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn fetch_client_code(request_line: &String) {
    let code = request_line
        .split("code=")
        .last()
        .unwrap()
        .split("&")
        .next()
        .unwrap();

    println!("{code}");
}
