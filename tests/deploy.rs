use dotenv::dotenv;
use fuels::prelude::{Provider, WalletUnlocked};
use src20_sdk::{DeployTokenConfig, deploy_token_contract};
const RPC: &str = "beta-3.fuel.network";
#[tokio::test]
async fn deploy() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let configs: Vec<DeployTokenConfig> = vec![
        DeployTokenConfig {
            name: String::from("Bitcoin"),
            symbol: String::from("BTC"),
            decimals: 8,
        },
        DeployTokenConfig {
            name: String::from("Uniswap"),
            symbol: String::from("UNI"),
            decimals: 9,
        },
        DeployTokenConfig {
            name: String::from("USD Coin"),
            symbol: String::from("USDC"),
            decimals: 6,
        },
    ];
    for config in configs {
        let bin_path = "contract/out/debug/FRC20.bin";
        let res = deploy_token_contract(&wallet, &config, bin_path).await;
        println!("Deployed {} {}", config.name, res.contract_id().hash);
    }
}
