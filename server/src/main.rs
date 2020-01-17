use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let contents;
    let response;

    if buffer.starts_with(get) {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        contents = fs::read_to_string("hello.html").expect("cannot read file");
        response = format!("{}{}", status_line, contents);
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        contents = fs::read_to_string("404.html").expect("cannot read file");
        response = format!("{}{}", status_line, contents);
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
