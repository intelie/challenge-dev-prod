use std::net::Ipv4Addr;
use tokio::net::TcpListener;

mod generator;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let bind_address: (Ipv4Addr, u16) = ("0.0.0.0".parse().unwrap(), 9999);
    let mut listener = TcpListener::bind(bind_address).await?;

    println!("Listening at tcp://{}:{}", bind_address.0, bind_address.1);
    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            generator::process_connection(socket).await;
        });
    }
}
