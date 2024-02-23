use dotenv::dotenv;
use fuels::prelude::{Provider, WalletUnlocked};
use src20_sdk::{constants::RPC, print_title, token_utils::deploy_token_contract};

#[tokio::main]
async fn main() {
    print_title("Deploy");

    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let contract = deploy_token_contract(&wallet).await;
    println!(
        "The token contract has been deployed {}\n",
        contract.contract_id().hash
    );
}
