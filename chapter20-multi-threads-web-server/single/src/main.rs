use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;

extern crate log;
extern crate stderrlog;
use log::{trace, debug, info, warn, error};

fn main() {
        // stderrlog
        stderrlog::new().module(module_path!()).verbosity(99).init().unwrap();

        trace!("Testing TCP connection.");

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
                let stream = stream.unwrap();  // get TCP stream connection from TCP request
                trace!("Tcp connection established!");
                handle_connection(stream);
        }
}

fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        // trace!("Request: {}.", String::from_utf8_lossy(&buffer));

        // response
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";
/*
        if buffer.starts_with(get) {
                trace!("Hit.");
                let header = "HTTP/1.1 200 OK\r\n\r\n";
                let body = fs::read_to_string("hello.html").unwrap();
                let response = format!("{}{}", header, body);
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
        } else {
                trace!("Not hit.");
                let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                let body = fs::read_to_string("404.html").unwrap();
                let response = format!("{}{}", header, body);
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
        }
*/

        let (header, body_file) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(50));
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };
        let response = format!("{}{}", header, fs::read_to_string(body_file).unwrap());
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}
