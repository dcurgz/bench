use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

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

async fn handle_connection(socket: TcpStream) -> io::Result<()> {
    println!("client connected");
    let (mut rd, mut wr) = io::split(socket);
    let mut buf = vec![0; 1024];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break Ok(());
        }

        println!("read {} bytes", n);
    }
}

