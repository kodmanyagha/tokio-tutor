mod redis_client;
mod tcp_server;

use anyhow::{anyhow, bail};
use mini_redis::{client, Buffer};
use redis_client::redis_client;
use tcp_server::tcp_server;
use tokio::time::{sleep, Duration};

async fn spawn_me() -> anyhow::Result<()> {
    loop {
        println!("spawn_me() > Loop cycle starting, waiting 1 second.");
        sleep(Duration::from_millis(1000)).await;
    }

    //Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start async function immediately but not wait it.
    tokio::spawn(async move {
        let _ = spawn_me().await;
    });

    //redis_client().await?;
    tcp_server().await?;

    Ok(())
}
