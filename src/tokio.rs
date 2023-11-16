use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Server running on 127.0.0.1:8080");

        loop {
            let (mut socket, addr) = listener.accept().await.unwrap();
            println!("New connection: {}", addr);
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    println!("{:#?}", buf);

                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprint!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                    println!("Echoed {} bytes", n);
                }
            });
        }
    })
}
