use tokio::io::{copy, split};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let local_addr = listener.local_addr()?;
    println!("Echo server listening on {}", local_addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            let (mut reader, mut writer) = split(socket);
            if let Err(e) = copy(&mut reader, &mut writer).await {
                eprintln!("Error copying data for {}: {}", addr, e);
            }
            println!("Connection closed by {}", addr);
        });
    }
}
