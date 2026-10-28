use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&stream);
                let http_request: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();
                let request_line = http_request[0].split_whitespace().skip(1).next();
                let message = match request_line {
                    Some("/") | None => "200 OK",
                    Some(path) => "404 Not Found",
                };

                println!("{:?}", http_request);
                let response = format!("HTTP/1.1 {message}\r\n\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
