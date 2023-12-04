use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established! {:#?}", stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request[0]);
    let mut response: String = "".to_owned();

    if http_request[0] == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let file = fs::read_to_string("index.html").unwrap();
        let length = file.len();
        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file}");
    } else {
        let status_line = "HTTP/1.1 404 Not Found";
        let file = fs::read_to_string("404.html").unwrap();
        let length = file.len();
        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file}");
    }
    stream.write_all(response.as_bytes()).unwrap();
}
