use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

extern crate mwaki_server;
use mwaki_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:20000").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        pool.execute(|| {
            handleConnection(stream);
        });
    }
}

fn handleConnection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (header, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let mut f = File::open(file_name).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
     
}
