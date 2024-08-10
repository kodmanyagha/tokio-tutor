use anyhow::anyhow;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

pub async fn tcp_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7788")
        .await
        .map_err(|_| anyhow!("Tcp server error."))?;

    println!("Redis server started at 127.0.0.1:7788");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket)
            .await
            .map_err(|_| anyhow!("Socket processing error."))?;
    }
}

async fn process(socket: TcpStream) -> anyhow::Result<()> {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection
        .read_frame()
        .await
        .map_err(|_| anyhow!("Frame can't read."))?
    {
        println!("Got {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

    Ok(())
}
