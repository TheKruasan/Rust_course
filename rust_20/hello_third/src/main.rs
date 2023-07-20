use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();//create new tcp lictener
    let pool = ThreadPool::new(4);//create 4 threads

    for stream in listener.incoming().take(2) {//do 2 request and end it
        let stream = stream.unwrap();//if we have error stop work

        pool.execute(|| {
            handle_connection(stream);//else start handle streams
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];//create buffer
    stream.read(&mut buffer).unwrap();//read from buffer

    let get = b"GET / HTTP/1.1\r\n";//get req
    let sleep = b"GET /sleep HTTP/1.1\r\n";//sleep req

    let (status_line, filename) = if buffer.starts_with(get) {// if we have just http://127.0.0.1:7878 we return the hello.html
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {// if we have just http://127.0.0.1:7878/sleep we return the hello.html with 5 sec of duration
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")//else we return 404.html
    };

    let contents = fs::read_to_string(filename).unwrap();//read the nessesary file

    let response = format!(//create response
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    //write you response to stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}