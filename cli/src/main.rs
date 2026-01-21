use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt,
    BufReader,       // main: read from stdin
    AsyncBufReadExt  // main: read from stdin
};
use tokio::net::{TcpStream};  // handle_connection
use std::io::{Write};  // main: stdout flush
use std::net::SocketAddr;
use protobuf::{EnumOrUnknown, Message}; // connect
 
use common::types::{PACKET_HELLO};
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

struct ConnectionContext {
    addr: SocketAddr,
}

fn assert_length(n: usize, argv: &Vec<&str>, err: &str) -> bool {
    if argv.len() < n {
        print!("E: {}\n", err);
        return false
    }
    return true
}

fn greet() {
    print!(":: bench CLI v0.1\n");
    print!("type 'help' for a list of commands\n");
    print!("\n");
}

fn help() {
    print!("commands list ...\n");
    print!("   help\n");
    print!("     Show this help message.\n");
    print!("   connect <ip>\n");
    print!("     Connect to the bench gateway at the given ip address.\n");
    print!("\n");
    print!("     Examples:\n");
    print!("       connect 192.168.0.34\n");
    print!("       connect 43.56.342.64:9983 ; use a non-default port\n");
    print!("   login <username>\n");
    print!("     Set your username for this session.\n");
    print!("\n");
    print!("     Examples:\n");
    print!("       login dcurgz\n");
    print!("   say <channel> [...message]\n");
    print!("     Send a message to the channel.\n");
    print!("\n");
    print!("     Examples:\n");
    print!("       say #public hi guys\n");
    print!("   read <channel>\n");
    print!("     Get recent messages from the channel.\n");
}

async fn connect(host: &str) -> std::io::Result<ConnectionContext> {
    print!("connecting to {host} ... ");
    let socket = TcpStream::connect(host).await?;
    let addr: SocketAddr = socket
        .peer_addr()
        .unwrap();
    println!("OK");
    tokio::spawn(handle_connection(socket));
    Ok(ConnectionContext{addr: addr})
}

async fn handle_connection(socket: TcpStream) {
    let addr = socket
        .peer_addr()
        .unwrap()
        .to_string();
    let (mut rd, mut wr) = tokio::io::split(socket);
    let mut buf = vec![0; 1024];

    let mut hello = Hello::new();
    hello.client_version = EnumOrUnknown::new(hello::Version::V0_1);
    let hello_bytes: Vec<u8> = hello.write_to_bytes().unwrap();
    wr.write_i32(PACKET_HELLO).await;
    wr.write_all(&hello_bytes).await;
    wr.flush().await;

    loop {
        let n = rd.read(&mut buf).await.unwrap();

        if n == 0 {
            break
        }

        println!("read {} bytes", n);
    }

    println!("disconnected");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    greet();
    let mut buf = String::new();

    let mut ctx: Option<ConnectionContext> = None;
    loop {
        // write prompt 
        //print!(">> ");
        //let _ = std::io::stdout().flush();
        // read command
        buf.clear();
        std::io::stdin().read_line(&mut buf)?;
        // parse
        let command = buf.as_str().trim();
        let argv    = command.split(" ").collect::<Vec<_>>();
        // match
        match argv[0] {
            "help" => {
                help();
            }
            "connect" => {
                if !assert_length(2, &argv, "usage: connect <ip>") {
                    continue;
                }
                ctx = Some(connect(argv[1]).await?);
            }
            "login" => {
            }
            _ => {
                print!("unrecognized command: {}\n", argv[0]);
            }
        }
    }
}
