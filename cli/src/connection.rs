use std::sync::mpsc::{channel, Sender, Receiver};

use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

use protobuf::Message;

use common::types::{
    PACKET_HELLO,
};

struct Connection {
}

impl Connection {
    pub fn new(mut rd: ReadHalf<AsyncRead>, mut wr: WriteHalf<AsyncWrite>) -> Self {
        let (rx, tx) = channel::<Message>();
        tokio::spawn(async move || reader(rd, tx);
        tokio::spawn(async move || sender(wr, rx);
    }
}

fn reader(mut rd: ReadHalf<AsyncRead>, tx: Sender<Message>) -> Result<()> {
    let mut buf = vec![0; 1024];
    loop {
        let id = rd.read_i32().await?;
        let n  = rd.read(&mut buf).await?;

        println!("read {} bytes", n);
        match id {
            PACKET_HELLO -> 
        }
    }
    println!("disconnected");
    Ok(())
}

fn writer(mut wr: WriteHalf<AsyncWrite>, rx: Receiver<Message>) -> Result<()> {
    loop {
        let msg = rx.recv()?;
    }
    Ok(())
}
