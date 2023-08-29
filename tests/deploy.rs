use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{AssetId, ContractId},
};
use src20_sdk::{
    deploy_token_factory_contract, token_factory_abi_calls, DeployTokenConfig, TokenFactoryContract,
};
const RPC: &str = "beta-4.fuel.network";

const FACTORY_ADDRESS: &str = "0xd8c627b9cd9ee42e2c2bd9793b13bc9f8e9aad32e25a99ea574f23c1dd17685a";

#[tokio::test]
async fn deploy() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let configs: Vec<DeployTokenConfig> = vec![
        DeployTokenConfig {
            name: String::from("USD Coin"),
            symbol: String::from("USDC"),
            decimals: 6,
        },
        DeployTokenConfig {
            name: String::from("Chainlink"),
            symbol: String::from("LINK"),
            decimals: 9,
        },
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
            name: String::from("Swaylend"),
            symbol: String::from("SWAY"),
            decimals: 9,
        },
        DeployTokenConfig {
            name: String::from("Compound"),
            symbol: String::from("COMP"),
            decimals: 9,
        },
        DeployTokenConfig {
            name: String::from("Spark"),
            symbol: String::from("SPARK"),
            decimals: 9,
        },
    ];

    let factory = if FACTORY_ADDRESS == "" {
        let bin_path = "contract/out/debug/token-factory.bin";
        let factory = deploy_token_factory_contract(&wallet, bin_path).await;
        println!(
            "The factory has been deployed {}",
            factory.contract_id().hash
        );
        factory
    } else {
        let id = ContractId::from_str(FACTORY_ADDRESS).unwrap();
        TokenFactoryContract::new(id, wallet.clone())
    };

    for config in configs {
        token_factory_abi_calls::deploy(&factory, &config.symbol, &config.name, config.decimals)
            .await
            .unwrap();
        let bits256 = token_factory_abi_calls::asset_id(&factory, &config.symbol)
            .await
            .unwrap()
            .value;
        println!("Deployed {} {}", config.name, AssetId::from(bits256.0));
    }
}
