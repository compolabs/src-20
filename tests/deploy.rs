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

const FACTORY_ADDRESS: &str = "0x8a25657aa845a67fec72a60e59ac01342483e89a5ef9215eb52c4e56270b082f";

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
            name: String::from("Compound"),
            symbol: String::from("COMP"),
            decimals: 9,
        }
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
        let asset_id = AssetId::from(bits256);
        println!("Deployed {}({}) {}", config.name, config.symbol, asset_id);
    }
}

/*
Deployed USD Coin(USDC) 8bf7951ea3222fe0bae9b811c2b142a1ff417361dcf7457855ed477d2d9a8550
Deployed Chainlink(LINK) fa36ce38f74ee0bcd12c11ec367ca237cee3e6b7a937761f4f762bbc11d2da21
Deployed Bitcoin(BTC) 49fab925448594b61c280b5c580b2a63a6f6a8aaa3c199a06798b1c568808158
Deployed Uniswap(UNI) ae37bc0feb66e60a89e301d450bb4640aa9bd7cedd856e253e23989eae536e92
Deployed Swaylend(SWAY) a715086b6bb8c944ba370c78e7ca38c35ec5c0f758fb2b57be11fdae23988600
Deployed Compound(COMP) c23da8f4073523e84e4fdd2b6a28f3273af2b70361345a63307313f0f489c48b
Deployed Spark(SPARK) 7b0c0ea5886c2699887f63983187a63954d5c41639c6b055894f487810136c03
*/
