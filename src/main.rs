use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {

    let port: u32 = 7878;
    let url: String = "localhost:".to_string();
    let adress: String = url + &port.to_string();

    println!("Connecting to server at {}", adress);

    match TcpStream::connect(adress) {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent '{}', awaiting reply...", from_utf8(msg).unwrap());

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        let reply = from_utf8(&data).unwrap();
                        println!("Reply is ok!");
                        println!("Reply: {}", reply);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}