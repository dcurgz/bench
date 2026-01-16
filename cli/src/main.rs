use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt,
    BufReader,       // main: read from stdin
    AsyncBufReadExt  // main: read from stdin
};
use tokio::net::TcpStream;
use std::io::{self, Write};  // main: stdout flush

struct ConnectionContext {
    socket: TcpStream,
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

async fn connect(host: &str) -> io::Result<ConnectionContext> {
    let parts = host.split(":").collect::<Vec<_>>();
    let ip    = parts[0];
    let port  = if parts.len() > 2 {
        parts[1].parse::<i32>().unwrap()
    } else {
        /* default = */ 9983
    };

    let server = format!("{}:{}", ip, port);
    println!("OK, connecting to {}", server);

    let socket = TcpStream::connect(server).await?;
    Ok(ConnectionContext{socket: socket})
}

#[tokio::main]
async fn main() -> io::Result<()> {
    greet();
    let mut buf = String::new();

    let mut ctx: Option<ConnectionContext> = None;
    loop {
        // write prompt 
        print!(">> ");
        let _ = io::stdout().flush();
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
