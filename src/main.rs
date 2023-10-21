use std::{
    fs,
    io::{prelude::*, BufReader, self},
    net::{TcpListener, TcpStream},
};

use http_server::ThreadPool;

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

struct HTTP {
    status_line: &'static str,
    filename: String
}

fn find_requested_file(path: &str, file_route: String) -> HTTP {
    /* Find requested file by walking the directory */
    let mut entries = fs::read_dir(path).expect("directory does not exist")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    entries.sort();

    let mut http = HTTP {
        status_line: "HTTP/1.1 404 NOT FOUND",
        filename: String::from("static/404.html"),
    };

    for entry in entries.clone() {
        let path = entry.to_str().unwrap();
        if path == file_route {
            http = HTTP {
                status_line: "HTTP/1.1 200 OK",
                filename: file_route,
            };
            return http;
        }
        else if entry.is_dir() {
            http = find_requested_file(entry.to_str().unwrap(), file_route.clone());
        }
    }

    return http;
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

    // walk static directory
    let http = find_requested_file("static", file_route);
    
    let contents = fs::read_to_string(http.filename).unwrap();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        http.status_line,
        contents.len(),
        contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    Ok(())
}
