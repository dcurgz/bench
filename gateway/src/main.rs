use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::io::BufReader; // handle_connection

use common::types::identity::{
    Hello,
    HelloAck,
    hello,
    Identify,
    IdentifyAck,
    identify_ack,
};
use common::types::chat::{
    GetMessages,
    GiveMessages,
    SendMessage,
    send_message_ack,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!(":: bench gateway v0.1");
    let ip = "0.0.0.0";
    let port = 9983;
    println!("serving on {}:{} ...", ip, port);

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(socket: TcpStream) {
    let addr = socket
        .peer_addr()
        .unwrap()
        .to_string();
    println!("{addr}> connected");
    let (mut rd, mut wr) = io::split(socket);
    let mut type_id: i32 = 0;
    let mut buf = vec![0; 1024];

    loop {
        // get packet type
        type_id = rd.read_i32().await.unwrap();

        match type_id {
            common::types::PACKET_HELLO => {
                println!("{addr}> Hello");
            }
            common::types::PACKET_IDENTIFY => {
                println!("{addr}> Identify");
            }
            common::types::PACKET_GET_MESSAGES => {
                println!("{addr}> GetMessages");
            }
            common::types::PACKET_SEND_MESSAGE => {
                println!("{addr}> SendMessage");
            }
            _ => {
                println!("{addr}> ???");
            }
        }

        let n = rd.read(&mut buf).await.unwrap();

        if n == 0 {
            break
        }

        println!("read {} bytes", n);
    }

    println!("{addr}> disconnected");
}
