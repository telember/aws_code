use clean_security_group::{scan};
use tokio;

mod clean_security_group;


#[tokio::main]
async fn main() -> () {
    scan(false).await;
}