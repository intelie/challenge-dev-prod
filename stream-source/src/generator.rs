use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn process_connection(mut stream:TcpStream) {
    let address = stream.peer_addr().unwrap();
    println!("{} - Connected!", address);

    let line = "Ok!\n".as_bytes();

    if let Err(e) = stream.write(line).await {
        eprintln!("{} - {}", address, e);
        return;
    }
}