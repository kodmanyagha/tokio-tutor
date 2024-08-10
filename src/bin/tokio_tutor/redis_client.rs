use anyhow::{anyhow, bail};
use mini_redis::{client, Buffer};

pub async fn redis_client() -> anyhow::Result<()> {
    let mut client = client::connect("127.0.0.1:6379")
        .await
        .map_err(|_| anyhow!("Connection problem occured."))?;

    client
        .set("hello", "world".into())
        .await
        .map_err(|_| anyhow!("set method error."))?;

    let result = client
        .get("hello")
        .await
        .map_err(|_| anyhow!("Can't get data."))?;

    println!("got value from the server: {:?}", result);

    Ok(())
}
