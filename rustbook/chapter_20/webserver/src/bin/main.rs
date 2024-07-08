use std::net::{
    TcpListener,
    TcpStream
};
use std::io::prelude::*;
use std::fs;
use std::io::BufReader;

use webserver::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind to socket");
    let pool = ThreadPool::new(5);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut response;

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        let contents = fs::read_to_string("html/index.html").unwrap();
        response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    } else {
        let contents = fs::read_to_string("html/404.html").unwrap();
        response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
    }
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}