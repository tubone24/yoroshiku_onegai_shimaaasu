// Refs
//https://qiita.com/taichitk/items/5661dda19661b1f4efaf

extern crate httparse;

use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;
use std::path::Path;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        thread::spawn(|| {
            handle_client(stream.unwrap())
        });
    }
}


fn handle_client(mut stream: TcpStream) {
    let mut buf = [0 ;4096];
    stream.read(&mut buf).unwrap();
    let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut parsed_headers);
    req.parse(&buf).unwrap();
    match req.path {
        Some(ref path) => {
            println!("GET: path {}", path);
            let mut body = String::new();
            let mut binaryBody: Vec::<u8>;
            if match_file(path) == "html" {
                let mut html = read_html_file(path);
                html.read_to_string(&mut body);
                let status = "HTTP/1.1 200 OK\r\n".to_string();
                let header = status + "Content-Type: text/html; charset=UTF-8\r\n\r\n";
                let res = header + &body + "\r\n";
                let data = res.as_bytes();
                stream.write(data);
            } else if match_file(path) == "css" {
                let mut css = read_other_file(path);
                css.read_to_string(&mut body);
                let status = "HTTP/1.1 200 OK\r\n".to_string();
                let header = status + "Content-Type: text/css; charset=UTF-8\r\n\r\n";
                let res = header + &body + "\r\n";
                let data = res.as_bytes();
                stream.write(data);
            } else if match_file(path) == "js" {
                let mut css = read_other_file(path);
                css.read_to_string(&mut body);
                let status = "HTTP/1.1 200 OK\r\n".to_string();
                let header = status + "Content-Type: text/javascript; charset=UTF-8\r\n\r\n";
                let res = header + &body + "\r\n";
                let data = res.as_bytes();
                stream.write(data);
            } else if match_file(path) == "wasm" {
                let mut css = read_other_file(path);
                css.read_to_end(&mut binaryBody);
//                let status = "HTTP/1.1 200 OK\r\n".to_string();
//                let header = status + "Content-Type: application/wasm; charset=UTF-8\r\n\r\n";
//                let res = header + &body + "\r\n";
//                let data = res.as_bytes();
                stream.write(&binaryBody);
            }
        },
        None => {
        }
    }
}

fn read_html_file(path: &&str) -> File {
    let html_file = match path {
        &"/" => "index.html".to_string(),
        _ => ".".to_string() + &path + ".html",
    };
    let html_file_path = Path::new(&html_file);
    File::open(&html_file_path).unwrap()
}

fn read_other_file(path: &&str) -> File {
    let css_file =  ".".to_string() + &path;
    let css_file_path = Path::new(&css_file);
    println!("Load {}", css_file);
    File::open(&css_file_path).unwrap()
}

fn match_file(path: &&str) -> String {
    if path.ends_with("/") {
        println!("FileType html");
        return String::from("html")
    } else if path.ends_with(".css") {
        println!("FileType css");
        return String::from("css")
    } else if path.ends_with(".js") {
        println!("FileType js");
        return String::from("js")
    } else if path.ends_with(".wasm") {
        println!("FileType wasm");
        return String::from("wasm")
    }
    return String::from("other")
}
