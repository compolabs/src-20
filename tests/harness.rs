use fuels::{
    prelude::ViewOnlyAccount,
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
    types::{AssetId, ContractId},
};
use src20_sdk::{deploy_token_contract, token_abi_calls, DeployTokenConfig};

#[tokio::test]
async fn main_test() {
    //wallets
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None).await;
    let admin = &wallets[0]; //token owner
    let alice = &wallets[1]; //token mint recipient

    //token deploy
    let token_config = &DeployTokenConfig {
        name: "Spark Token".to_owned(),
        symbol: "SPARK".to_owned(),
        decimals: 9,
    };
    let token = deploy_token_contract(admin, token_config, "contract/out/debug/FRC20.bin").await;
    let asset_id = AssetId::from(*ContractId::from(token.contract_id()));

    //mint
    let mint_amount = 1000_000_000_000;
    token_abi_calls::mint(&token, mint_amount, alice.address().into())
        .await
        .unwrap();

    //check if mint is ok
    // assert!(alice.get_asset_balance(&asset_id).await.unwrap() == mint_amount);
}
