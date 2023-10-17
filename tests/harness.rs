use fuels::{
    prelude::ViewOnlyAccount,
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
    types::AssetId,
};
use src20_sdk::{deploy_token_factory_contract, token_factory_abi_calls};

#[tokio::test]
async fn main_test() {
    //wallets
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None).await;
    let admin = &wallets[0]; //token owner
    let alice = &wallets[1]; //token mint recipient

    // factory deploy
    let bin_path = "contract/out/debug/token-factory.bin";
    let factory = deploy_token_factory_contract(&admin, bin_path).await;
    // println!(
    //     "The factory has been deployed {}",
    //     factory.contract_id().hash
    // );

    // let is_err = token_factory_abi_calls::name(&factory, "SPARK")
    //     .await
    //     .is_err();
    // assert!(is_err);

    let is_err = token_factory_abi_calls::decimals(&factory, "SPARK")
        .await
        .is_err();
    assert!(is_err);

    let is_err = token_factory_abi_calls::admin(&factory, "SPARK")
        .await
        .is_err();
    assert!(is_err);

    let is_err = token_factory_abi_calls::total_supply(&factory, "SPARK")
        .await
        .is_err();
    assert!(is_err);

    let is_err = token_factory_abi_calls::mint(&factory, alice.address().into(), "SPARK", 1)
        .await
        .is_err();
    assert!(is_err);

    //token deploy
    token_factory_abi_calls::deploy(&factory, "SPARK", "Spark Token", 9)
        .await
        .unwrap();

    let is_err = token_factory_abi_calls::deploy(&factory, "SPARK", "Spark Token", 9)
        .await
        .is_err();
    assert!(is_err);

    let bits256 = token_factory_abi_calls::asset_id(&factory, "SPARK")
        .await
        .unwrap()
        .value;
    let asset_id = AssetId::from(bits256.0);

    //mint
    let mint_amount = 1000_000_000_000;
    token_factory_abi_calls::mint(&factory, alice.address().into(), "SPARK", mint_amount)
        .await
        .unwrap();

    //check if mint is ok
    assert!(alice.get_asset_balance(&asset_id).await.unwrap() == mint_amount);
}
