// Refs
//https://qiita.com/taichitk/items/5661dda19661b1f4efaf

extern crate chunked_transfer;
extern crate httparse;

use chunked_transfer::Encoder;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

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
    let mut buf = [0; 4096];
    stream.read(&mut buf).unwrap();
    let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut parsed_headers);
    req.parse(&buf).unwrap();
    match req.path {
        Some(ref path) => {
            println!("GET: path {}", path);
            let mut body = String::new();
            let mut binaryBody = Vec::new();
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
                let mut wasm = read_other_file(path);
                wasm.read_to_end(&mut binaryBody);
                let headers = [
                    "HTTP/1.1 200 OK",
                    "Content-type: application/wasm",
                    "\r\n"
                ];
                let mut response = headers.join("\r\n")
                    .to_string()
                    .into_bytes();
                response.extend(binaryBody);
                match stream.write(&response) {
                    Ok(_) => println!("Response sent"),
                    Err(e) => println!("Failed sending response: {}", e),
                }
            } else if match_file(path) == "ico" {
                let mut wasm = read_other_file(path);
                wasm.read_to_end(&mut binaryBody);
                let headers = [
                    "HTTP/1.1 200 OK",
                    "Content-type: image/vnd.microsoft.icon",
                    "\r\n"
                ];
                let mut response = headers.join("\r\n")
                    .to_string()
                    .into_bytes();
                response.extend(binaryBody);
                match stream.write(&response) {
                    Ok(_) => println!("Response sent"),
                    Err(e) => println!("Failed sending response: {}", e),
                }
            }
        }
        None => {}
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
    let file = ".".to_string() + &path;
    let css_file_path = Path::new(&file);
    println!("Load {}", file);
    File::open(&css_file_path).unwrap()
}

fn match_file(path: &&str) -> String {
    if path.ends_with("/") {
        println!("FileType html");
        return String::from("html");
    } else if path.ends_with(".css") {
        println!("FileType css");
        return String::from("css");
    } else if path.ends_with(".js") {
        println!("FileType js");
        return String::from("js");
    } else if path.ends_with(".wasm") {
        println!("FileType wasm");
        return String::from("wasm");
    } else if path.ends_with(".ico") {
        println!("FileType ico");
        return String::from("ico");
    }
    return String::from("other");
}
