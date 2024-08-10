#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3344")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/").await?.print().await?;

    Ok(())
}
