#![allow(dead_code)]

use crate::{todo_persistency_dummy::create_empty_todo_persistency, types::traits::persistency::*};

use crate::todo_dto::TodoDTO;

use std::{cell::RefCell, error::Error, net::TcpStream};
use stream::MaybeTlsStream;
use tungstenite::{
    protocol::{frame::coding::CloseCode::*, CloseFrame},
    *,
};
use url::Url;

#[derive(Debug)]
struct TodoPersistencyWS {
    socket: RefCell<WebSocket<MaybeTlsStream<TcpStream>>>,
}

pub fn create_todo_ws_persistency(address: &str) -> Box<dyn IPeristency<TodoDTO>> {
    match create(address) {
        Ok(persistency) => Box::new(persistency),
        Err(_) => create_empty_todo_persistency(),
    }
}

fn create(address: &str) -> Result<TodoPersistencyWS, Box<dyn Error>> {
    let url = Url::parse(address)?;
    let (socket, response) = connect(url)?;

    let ref_socket = RefCell::from(socket);

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let persistency = TodoPersistencyWS { socket: ref_socket };
    Ok(persistency)
}

impl IPeristency<TodoDTO> for TodoPersistencyWS {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        let msg = self.socket.borrow_mut().read()?;
        let mut result: Vec<TodoDTO> = vec![];

        if let Message::Binary(bin_msg) = msg {
            result = serde_json::from_slice(&bin_msg)?;
        }

        Ok(result)
    }

    fn save(&self, data: &[TodoDTO]) -> Result<(), Box<dyn Error>> {
        let result = serde_json::to_vec(data)?;
        self.socket.borrow_mut().send(Message::Binary(result))?;

        Ok(())
    }
}

impl Drop for TodoPersistencyWS {
    fn drop(&mut self) {
        self.socket
            .borrow_mut()
            .close(Some(CloseFrame {
                code: Normal,
                reason: "End".into(),
            }))
            .expect("Error while closing connection.");
    }
}
