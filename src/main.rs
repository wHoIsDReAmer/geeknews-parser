mod client;

use std::time::Duration;
use crate::client::hada::HadaGeekNews;
use crate::client::Parser;
use crate::client::velog::Velog;

const HADA_WEBHOOK: &'static str = "YOUR_WEBHOOK_HERE";
const VELOG_WEBHOOK: &'static str = "YOUR_WEBHOOK_HERE";

#[tokio::main]
async fn main() {
    println!("News Parser");

    let hada = HadaGeekNews::new(HADA_WEBHOOK);
    let velog = Velog::new(VELOG_WEBHOOK);

    let _ = tokio::join!(hada.ticker());
}