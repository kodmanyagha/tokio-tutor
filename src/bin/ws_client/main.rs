use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio_websockets::{ClientBuilder, Error, Message};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let uri = Uri::from_static("wss://stream.binance.com:443");
    let (mut client, _) = ClientBuilder::from_uri(uri).connect().await?;

    client
        .send(Message::text("Hello, world!".to_string()))
        .await?;

    while let Some(item) = client.next().await {
        println!("{item:?}");
    }

    Ok(())
}
