use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Gagal bind ke alamat 127.0.0.1:7878");

    println!("Server berjalan di http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.expect("Gagal menerima koneksi");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<String> = buf_reader
        .lines()
        .map(|line| line.expect("Gagal membaca baris"))
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html")
        .expect("Gagal membaca file hello.html");
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes())
        .expect("Gagal menulis respon ke stream");
}
