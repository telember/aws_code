use scan_security_group::{scan};
use tokio;

mod scan_security_group;


#[tokio::main]
async fn main() -> () {
    scan().await;
}