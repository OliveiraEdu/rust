use ipfs_api::{IpfsApi, IpfsClient, Scheme};
use std::io::Cursor;

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::from_host_and_port(
        Scheme::Http,
        "192.168.1.100",
        5001
    ).expect("Failed to create IPFS client");

    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
