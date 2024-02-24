use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;
use actix_rt::Runtime;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Replace "http://localhost:5001" with your actual IPFS node address
    let client = IpfsClient::new("http://localhost:5001");
    let data = Cursor::new("Hello World!");

    let rt = Runtime::new()?;

    let res = rt.block_on(client.add(data)).await?;

    println!("{}", res.hash);

    Ok(())
}
