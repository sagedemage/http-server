use std::{
    fs,
    io::{prelude::*, BufReader, self},
    net::{TcpListener, TcpStream},
};

use http_server::ThreadPool;
use walkdir::WalkDir;

fn main() { 
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            let _ = handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), io::Error>{
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request = &request_line;
    let request_info = request.split(" ").collect::<Vec<&str>>();

    let mut file_route: String = "static".to_owned() + request_info[1];

    if file_route.ends_with("/") {
        file_route += "index.html";
    }

    println!("{}", file_route);

    let mut success = false;
    let mut status_line = "";
    let mut filename = String::new();

    for entry in WalkDir::new("static").min_depth(1) {
        if entry?.path().to_str().unwrap() == file_route {
            success = true;
            status_line = "HTTP/1.1 200 OK";
            filename = file_route;
            break;
        }
    }

    if !success {
        filename = String::from("static/404.html");
        status_line = "HTTP/1.1 404 NOT FOUND";
    }

    let contents = fs::read_to_string(filename).unwrap();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    Ok(())
}
