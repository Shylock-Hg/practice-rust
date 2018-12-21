use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

extern crate log;
extern crate stderrlog;
use log::{trace, debug, info, warn, error};

use multiple::ThreadPool;

fn main() {
        // stderrlog
        stderrlog::new().module(module_path!()).verbosity(99).init().unwrap();

        let pool = ThreadPool::new(4).unwrap();  // create a threads pool with 4 thread

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
                let stream = stream.unwrap();  // get TCP stream connection from TCP request
                trace!("Tcp connection established!");
/*
                // 1. unlimited thread count will be unsecure for DDoS.
                // 2. create thread when request will cost time to response.
                thread::spawn(|| {
                        handle_request(stream);});
*/
                // apply the dealing of request to one available thread or waiting for thread to be available
                pool.apply(|| {
                        handle_request(stream);
                });
        }
        
}

fn handle_request(mut stream: TcpStream) {  // handle a http/tcp request
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        // trace!("Request: {}.", String::from_utf8_lossy(&buffer));

        // response
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

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

