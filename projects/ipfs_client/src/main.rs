use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;

const CLIENT: IpfsClient = IpfsClient::new("http://10.0.0.100:5001");

#[actix_rt::main]
async fn main() {
    let data = Cursor::new("Hello World!");

    match CLIENT.add(data).await { // Use the const client directly
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
