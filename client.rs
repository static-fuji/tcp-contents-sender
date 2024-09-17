use std::fs::File;
use std::io::{self, copy};
use std::net::TcpStream;
use std::env;
use std::process;

fn response(mut dst: TcpStream, mut src: File) -> io::Result<()> {
    copy(&mut src, &mut dst)?;
    Ok(())
}

fn main() -> io::Result<()> {
    // コマンドライン引数の確認
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: file name is required. USAGE: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    // TCP接続を確立する
    let ip = "localhost"; 
    let port = 8080; 
    let addr = format!("{}:{}", ip, port); // ipアドレスとポート番号を設定
    let conn = TcpStream::connect(addr.as_str())?;
    println!("LOG: Connected to {}", addr.as_str());

    // ファイルを開く
    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to open file: {}", err);
        process::exit(1);
    });

    // ファイルの内容をTCPストリームに送信
    response(conn, file)?;
    println!("LOG: done");

    Ok(())
}