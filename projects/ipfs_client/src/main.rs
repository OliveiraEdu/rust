use ipfs_client::{IpfsClient, AddResult};
use std::fs::File;
use std::io::{Error, Read};

fn main() -> Result<(), Error> {
    // Replace with your Kubo IPFS daemon address
    let ipfs_address = "http://10.0.0.100:5001";

    // Create an IPFS client
    let mut ipfs = IpfsClient::new(ipfs_address)?;

    // Read the file to upload
    let mut file_data = Vec::new();
    let mut file = File::open("path/to/your/file.txt")?;
    file.read_to_end(&mut file_data)?;

    // Use the "add" endpoint of the Kubo RPC API
    let result: AddResult = ipfs.add(file_data)?;

    // Print the uploaded file's CID
    println!("Uploaded file CID: {}", result.hash);

    Ok(())
}
