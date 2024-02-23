use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, ContractId},
};
use src20_sdk::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    print_title,
    token_utils::{Asset, TokenContract},
};

const SYMBOL: &str = "USDC";
const AMOUNT: u64 = 100;
const RECIPIENT: &str = "0x194c4d5d321ea3bc2e87109f4a86520ad60f924998f67007d487d3cc0acc45d2";

#[tokio::main]
async fn main() {
    print_title("Mint");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let token_contarct = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let asset = Asset::new(wallet.clone(), token_contarct.contract_id().into(), SYMBOL);

    asset
        .mint(
            Address::from_str(RECIPIENT).unwrap(),
            asset.parse_units(AMOUNT as f64) as u64,
        )
        .await
        .unwrap();

    let asset_id = asset.asset_id;
    println!("{AMOUNT} {SYMBOL} ({asset_id}) has been minted to {RECIPIENT}\n");
}
