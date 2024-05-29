#![allow(dead_code)]

use crate::types::traits::persistency::*;

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
    Box::new(create(address))
}

fn create(address: &str) -> TodoPersistencyWS {
    let (socket, response) = connect(Url::parse(address).unwrap()).expect("Can't connect");

    let ref_socket = RefCell::from(socket);

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    TodoPersistencyWS { socket: ref_socket }
}

impl IPeristency<TodoDTO> for TodoPersistencyWS {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        todo!()
    }

    fn save(&self, data: &[TodoDTO]) -> Result<(), Box<dyn Error>> {
        todo!()
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
