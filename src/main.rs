use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let tcp_listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);

    let request_status = buffer_reader.lines().next().unwrap().unwrap();

    let endpoint_path = request_status.split(' ').collect::<Vec<_>>()[1];

    let (status, filename) = if endpoint_path != "/" {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    } else {
        ("HTTP/1.1 200 OK", "src/hello-from-rust.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
