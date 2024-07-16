#![allow(unused)]

use anyhow::Result;


#[tokio::test]
async fn tester_http () -> Result<()>{
    let th = httpc_test::new_client ("http://localhost:8080")?;
    
    th.do_get("/hello?name=raw").await?.print().await?;
    th.do_get("/hello/taqi").await?.print().await?;
    // th.do_get("/src/main.rs").await?.print().await?;   
    Ok(())
}
 