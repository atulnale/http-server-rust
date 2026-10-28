#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    iter::Skip,
    ops::Index,
};

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
                let mut content = String::new();
                let message: String = match request_line {
                    Some("/") | None => String::from("200 OK"),
                    Some(path) => {
                        if path.starts_with("/echo/") {
                            let start = path.find("/echo/");
                            match start {
                                Some(x) => content = path[x + 6..].to_string(),
                                None => content = String::new(),
                            }
                            String::from("200 OK")
                        } else if path.starts_with("/user-agent") {
                            let mut agent = String::new();
                            for ele in http_request.iter() {
                                if ele.starts_with("User-Agent") {
                                    agent = ele.split(":").nth(1).unwrap_or("").trim().to_string();
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            content = agent;
                            String::from("200 OK")
                        } else {
                            String::from("404 Not Found")
                        }
                    }
                };

                println!("{:?}", http_request);
                println!("{content}");
                let response = format!(
                    "HTTP/1.1 {message}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
