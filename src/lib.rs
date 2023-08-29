use fuels::accounts::fuel_crypto::coins_bip32::prelude::k256::sha2::{Digest, Sha256};
use fuels::types::{AssetId, Bits256};
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Contract, LoadConfiguration, TxParameters},
};

abigen!(Contract(
    name = "TokenFactoryContract",
    abi = "contract/out/debug/token-factory-abi.json"
));

#[derive(Clone)]
pub struct DeployTokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
}

pub async fn deploy_token_factory_contract(
    wallet: &WalletUnlocked,
    bin_path: &str,
) -> TokenFactoryContract<WalletUnlocked> {
    // let configurables = TokenFactoryContractConfigurables::new();
    let config = LoadConfiguration::default(); //.set_configurables(configurables);
    let id = Contract::load_from(bin_path, config)
        .unwrap()
        .deploy(wallet, TxParameters::default().set_gas_price(1))
        .await
        .unwrap();

    TokenFactoryContract::new(id, wallet.clone())
}

pub mod token_factory_abi_calls {

    use fuels::{
        prelude::TxDependencyExtension,
        programs::call_response::FuelCallResponse,
        types::{Address, Bits256, Identity, SizedAsciiString},
    };

    use super::*;

    pub async fn asset_id(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
    ) -> Result<FuelCallResponse<Bits256>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory.methods().asset_id(symbol_hash).simulate().await
    }

    pub async fn total_supply(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
    ) -> Result<FuelCallResponse<u64>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory.methods().total_supply(symbol_hash).simulate().await
    }

    pub async fn decimals(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
    ) -> Result<FuelCallResponse<u8>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory.methods().decimals(symbol_hash).simulate().await
    }

    pub async fn name(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
    ) -> Result<FuelCallResponse<SizedAsciiString<32>>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory.methods().name(symbol_hash).simulate().await
    }
    pub async fn admin(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
    ) -> Result<FuelCallResponse<Identity>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory.methods().admin(symbol_hash).simulate().await
    }

    pub async fn mint(
        factory: &TokenFactoryContract<WalletUnlocked>,
        recipient: Address,
        symbol: &str,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);
        factory
            .methods()
            .mint(recipient, symbol_hash, amount)
            .tx_params(TxParameters::default().set_gas_price(1))
            .append_variable_outputs(1)
            .call()
            .await
    }
    pub async fn deploy(
        factory: &TokenFactoryContract<WalletUnlocked>,
        symbol: &str,
        name: &str,
        decimals: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let symbol_hash = get_symbol_hash(symbol);

        let mut name = name.to_string();
        name.push_str(" ".repeat(32 - name.len()).as_str());
        let name = SizedAsciiString::<32>::new(name.clone()).unwrap();

        factory
            .methods()
            .deploy(symbol_hash, name, decimals as u8)
            .tx_params(TxParameters::default().set_gas_price(1))
            .append_variable_outputs(1)
            .call()
            .await
    }
}

fn get_symbol_hash(symbol: &str) -> Bits256 {
    let mut hasher = Sha256::new();
    hasher.update(symbol);
    let symbol_hash: [u8; 32] = hasher.finalize().into();
    let hash_asset_id = AssetId::from(symbol_hash);
    Bits256::from(hash_asset_id)
}
