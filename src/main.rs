#![allow(unused_imports)]
use std::{

    io::{Read, Write},

    net::TcpListener,

};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

                println!("accepted new connection");

                let mut buf = [0; 512];

                loop {
                    match stream.read(&mut buf) {
                        Ok(0) => {
                            // إذا كانت القراءة تعيد 0، فإن العميل قد أغلق الاتصال
                            println!("Client disconnected");
                            break;
                        }
                        Ok(_) => {
                            // الرد على الطلب بإرسال PONG
                            stream.write(b"+PONG\r\n").unwrap();
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
