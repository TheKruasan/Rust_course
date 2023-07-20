use hello_sec::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();//create new tcp lictener
    let pool = ThreadPool::new(4);//create 4 threads
    for stream in listener.incoming() {//iterate on threads
        let stream = stream.unwrap();//if we have error stop work

        pool.execute(|| {
            handle_connection(stream);//else start handle streams
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);//create buf reader
    let request_line = buf_reader.lines().next().unwrap().unwrap();//pick the first request in buf reader and unwrap it...
                                                                           //1 unwrap for Option 2 unwrap for Result 

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),// if we have just http://127.0.0.1:7878 we return the hello.html
        "GET /sleep HTTP/1.1" => {// if we have just http://127.0.0.1:7878/sleep we return the hello.html with 5 sec of duration
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),//else we return 404.html
    };


    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");//create response

    stream.write_all(response.as_bytes()).unwrap();//write you response to stream
}
