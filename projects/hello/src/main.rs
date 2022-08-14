use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let response = match http_request.get(0).map(|s| &s[..]) {
        Some("GET / HTTP/1.1") => HttpResponse::from_file("index.html"),
        Some("GET /sleep HTTP/1.1") => {
            thread::sleep(Duration::from_secs(5));
            HttpResponse::from_file("index.html")
        }
        _ => HttpResponse::from_file_and_status(HttpStatus::NotFound, "404.html"),
    };

    stream.write_all(response.to_string().as_bytes()).unwrap();
}

enum HttpStatus {
    Ok,
    NotFound,
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => String::from("200 OK"),
            Self::NotFound => String::from("404 Not Found"),
        }
    }
}

struct HttpResponse<'a> {
    status: HttpStatus,
    headers: HashMap<&'a str, String>,
    body: String,
}

impl<'a> HttpResponse<'a> {
    fn from_file(path: &str) -> HttpResponse {
        let content = fs::read_to_string(path).unwrap();
        let mut headers = HashMap::new();

        headers.insert("Content-Length", content.len().to_string());

        HttpResponse {
            status: HttpStatus::Ok,
            headers,
            body: content,
        }
    }

    fn from_file_and_status(status: HttpStatus, path: &str) -> HttpResponse {
        let content = fs::read_to_string(path).unwrap();
        let mut headers = HashMap::new();

        headers.insert("Content-Length", content.len().to_string());

        HttpResponse {
            status,
            headers,
            body: content,
        }
    }
}

impl<'a> ToString for HttpResponse<'a> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result += &format!("HTTP/1.1 {}\r\n", self.status.to_string());

        for (key, value) in &self.headers {
            result += &format!("{}: {}\r\n", key, value);
        }

        result += "\r\n";

        result += &self.body;

        result
    }
}
