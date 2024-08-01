use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::request::{Request};

pub struct Server {
    addr: String,
    listener: Arc<Mutex<TcpListener>>,
}

impl Server {
    pub fn create(addr: &str) -> Server {
        let listener: Arc<Mutex<TcpListener>> = match TcpListener::bind(addr) {
            Ok(listener) => Arc::new(Mutex::new(listener)),
            Err(e) => {
                eprintln!("Failed to bind to addr: \"{}\" because: \"{}\"", addr, e);
                exit(1);
            },
        };

        println!("Server is bound on \"{}\"", addr);

        Server {
            addr: addr.to_string(),
            listener,
        }
    }

    fn handle_stream(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request = match Request::parse(http_request) {
            Ok(request) => request,
            Err(e) => {
                eprintln!("{e} stopping thread!");
                exit(1);
            }
        };
    }

    pub fn watch(&self) {
        let listener = self.listener.clone();
        for stream in listener.lock().expect("Failed to lock mutex").incoming() {
            match stream {
                Ok(stream) => {
                    let handler = self.clone();
                    thread::spawn(move || {
                        handler.handle_stream(stream);
                    });
                },
                Err(e) => {
                    eprintln!("Something went wrong with a stream: \"{}\", continuing anyway", e);
                }
            }
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            addr: self.addr.clone(),
            listener: Arc::clone(&self.listener),
        }
    }
}