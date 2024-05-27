use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read().unwrap();
                match msg {
                    tungstenite::Message::Text(text) => {
                        //websocket.send(msg).unwrap();

                        println!("{}", text);
                    }
                    tungstenite::Message::Binary(binary) => {
                        println!("{:?}", binary);
                    }
                    tungstenite::Message::Ping(message) => {
                        println!("{:?}", message);
                    }
                    tungstenite::Message::Pong(message) => {
                        println!("{:?}", message);
                    }
                    tungstenite::Message::Close(message) => {
                        println!("{:?}", message);
                    }
                    tungstenite::Message::Frame(message) => {
                        println!("{:?}", message);
                    }
                }
            }
        });
    }
}
