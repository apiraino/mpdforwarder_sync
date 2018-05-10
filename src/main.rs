#![deny(warnings)]

extern crate bufstream;
extern crate encoding;

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

use std::thread::spawn;
use std::net::TcpListener;

use encoding::{EncoderTrap, Encoding};
use encoding::all::ASCII;
use bufstream::BufStream;

fn handle_client(stream: &mut BufStream<TcpStream>) {
    let socket = TcpStream::connect("127.0.0.1:6600").unwrap();
    let mut reader = BufReader::new(&socket);
    let mut writer = BufWriter::new(&socket);
    let mut cmd = String::new();
    let mut response = String::new();

    // read response on server connect
    reader.read_line(&mut response).unwrap();
    println!(
        "[connect] response ({} bytes):\n{}",
        response.len(),
        response
    );

    // read cmd sent by client
    stream.read_line(&mut cmd).unwrap();
    println!("[cmd] {}", cmd);

    // send formatted cmd to server
    let mut command_bytes = ASCII.encode(&cmd, EncoderTrap::Strict).unwrap();
    command_bytes.push('\r' as u8);
    command_bytes.push('\n' as u8);
    writer.write(&command_bytes).unwrap();
    writer.flush().unwrap();

    // read server response
    let mut line;
    loop {
        line = String::new();
        reader.read_line(&mut line).unwrap();
        response.push_str(&line);
        if line == "OK\n" {
            break;
        }
    }
    println!(
        "[connect] response ({} bytes):\n{}",
        response.len(),
        response
    );
}

fn main() {
    let addr: String = "127.0.0.1:6601".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Server listening on tcp://{}", addr);

    for stream in listener.incoming() {
        match stream {
            Err(_) => println!("Error in stream, die."),
            Ok(stream_content) => {
                println!(
                    "stream from {} to {}",
                    stream_content.peer_addr().unwrap(),
                    stream_content.local_addr().unwrap()
                );
                spawn(move || {
                    let mut stream_content = BufStream::new(stream_content);
                    handle_client(&mut stream_content);
                });
            }
        }
    }
}
