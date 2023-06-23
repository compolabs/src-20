use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Contract, LoadConfiguration, TxParameters},
    types::SizedAsciiString,
};

abigen!(Contract(
    name = "TokenContract",
    abi = "contract/out/debug/FRC20-abi.json"
));
pub struct DeployTokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

pub async fn deploy_token_contract(
    wallet: &WalletUnlocked,
    deploy_config: &DeployTokenConfig,
    bin_path: &str,
) -> TokenContract<WalletUnlocked> {
    let mut name = deploy_config.name.clone();
    name.push_str(" ".repeat(32 - name.len()).as_str());
    let name = SizedAsciiString::<32>::new(name).unwrap();

    let mut symbol = deploy_config.symbol.clone();
    symbol.push_str(" ".repeat(8 - symbol.len()).as_str());
    let symbol = SizedAsciiString::<8>::new(symbol).unwrap();

    let configurables = TokenContractConfigurables::new()
        .set_DECIMALS(deploy_config.decimals)
        .set_NAME(name)
        .set_SYMBOL(symbol);
        // .set_OWNER(wallet.address().into());

    let config = LoadConfiguration::default().set_configurables(configurables);
    let id = Contract::load_from(bin_path, config)
        .unwrap()
        .deploy(wallet, TxParameters::default())
        .await
        .unwrap();

    TokenContract::new(id, wallet.clone())
}

pub mod token_abi_calls {

    use fuels::{programs::call_response::FuelCallResponse, types::Address};

    use super::*;

    pub async fn mint(
        instance: &TokenContract<WalletUnlocked>,
        amount: u64,
        recipient: Address,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        instance
            .methods()
            ._mint(amount, recipient)
            .tx_params(TxParameters::default().set_gas_price(1))
            .append_variable_outputs(1)
            .call()
            .await
    }
}
