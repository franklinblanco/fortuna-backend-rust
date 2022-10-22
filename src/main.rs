use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Payload;

// {"jsonrpc":"2.0","id":715578353668,"method":"abci_query","params":{"path":"/cosmwasm.wasm.v1.Query/SmartContractState","data":"0a0973746172736161616112497b226d696e745f636f756e74223a7b2261646472657373223a227374617273316863306832643677353977367172387a76657471716b72397a356b6a68646d6a3838656a3265227d7d","prove":false}}
#[tokio::main]
async fn main() {
    run();
}

fn run() {
    let _client = Client::new();
}