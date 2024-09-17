use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

const BUFF_SIZE: usize = 1024;

fn handle_error(err: std::io::Result<()>) {
    if let Err(e) = err {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

// TCP コネクションを受け取る
fn receive_tcp_connection() -> std::io::Result<()> {
    let ip = "0.0.0.0"; // 0.0.0.0 はすべてのアドレスを受け入れる
    let port = 8080; 
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addr.as_str())?;
    println!("Listening on port {}", port);

    // 接続を待ち受けて、接続があればエコーハンドラに渡す
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_error(echo_handler(stream));
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

// TCP コネクションからデータを受信して標準出力に表示
fn echo_handler(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; BUFF_SIZE];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!("Received: {:?}", &buf[..bytes_read]);
    }
}

fn main() {
    handle_error(receive_tcp_connection());
}
