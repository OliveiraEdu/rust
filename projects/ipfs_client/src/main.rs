use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}