contract;
use std::address::Address;
use std::constants::ZERO_B256;
use std::token::mint_to_address;
use std::bytes::Bytes;
use std::hash::sha256;
use std::call_frames::contract_id;

// configurable {
//     OWNER: Address = Address::from(ZERO_B256),
// }

storage {
    total_supply: StorageMap<b256, u64> = StorageMap::<b256, u64> {},
    decimals: StorageMap<b256, u8> = StorageMap::<b256, u8> {},
    name: StorageMap<b256, str[32]> = StorageMap::<b256, str[32]> {},
    admin: StorageMap<b256, Identity> = StorageMap::<b256, Identity> {},
}

abi TokenFactory {
    fn asset_id(symbol_hash: b256) -> AssetId;
    #[storage(read)]
    fn total_supply(symbol_hash: b256) -> u64;
    #[storage(read)]
    fn decimals(symbol_hash: b256) -> u8;
    #[storage(read)]
    fn name(symbol_hash: b256) -> str[32];
    #[storage(read)]
    fn admin(symbol_hash: b256) -> Identity;
    
    #[storage(read, write)]
    fn mint(recipient: Address, symbol_hash: b256, amount: u64);   

    #[storage(read, write)]
    fn deploy(symbol_hash: b256, name: str[32], decimals: u8);
}

enum Errors {
    TokenIsNotDeployed: (),
    TokenAlreadyDeployed: (),
    // AccessDenied: (),
}

impl TokenFactory for Contract {
    fn asset_id(symbol_hash: b256) -> AssetId {
        sha256((contract_id(), symbol_hash))
    }
    #[storage(read)]
    fn total_supply(symbol_hash: b256) -> u64 {
        let asset_id = sha256((contract_id(), symbol_hash));
        let total_supply = storage.total_supply.get(asset_id).try_read();
        require(total_supply.is_some(), Errors::TokenIsNotDeployed);
        total_supply.unwrap()
    }
    #[storage(read)]
    fn decimals(symbol_hash: b256) -> u8 {
        let asset_id = sha256((contract_id(), symbol_hash));
        let decimals = storage.decimals.get(asset_id).try_read();
        require(decimals.is_some(), Errors::TokenIsNotDeployed);
        decimals.unwrap()
    }
    #[storage(read)]
    fn name(symbol_hash: b256) -> str[32] {
        let asset_id = sha256((contract_id(), symbol_hash));
        let name = storage.name.get(asset_id).try_read();
        require(name.is_some(), Errors::TokenIsNotDeployed);
        name.unwrap()
    }
    #[storage(read)]
    fn admin(symbol_hash: b256) -> Identity {
        let asset_id = sha256((contract_id(), symbol_hash));
        let admin = storage.admin.get(asset_id).try_read();
        require(admin.is_some(), Errors::TokenIsNotDeployed);
        admin.unwrap()
    }

    #[storage(read, write)]
    fn mint(recipient: Address, symbol_hash: b256, amount: u64){
        let asset_id = sha256((contract_id(), symbol_hash));
        let admin = storage.admin.get(asset_id).try_read();
        require(admin.is_some(), Errors::TokenIsNotDeployed);
        // let admin = admin.unwrap();
        // require(msg_sender().unwrap() == admin || msg_sender().unwrap() == Identity::Address(OWNER), Errors::AccessDenied);
        let total_supply = storage.total_supply.get(asset_id).try_read().unwrap_or(0) + amount;
        storage.total_supply.insert(asset_id, total_supply);
        mint_to_address(recipient, symbol_hash, amount);
    }

    #[storage(read, write)]
    fn deploy(symbol_hash: b256, name: str[32], decimals: u8){
        let caller = msg_sender().unwrap();
        let asset_id = sha256((contract_id(), symbol_hash));
        require(storage.admin.get(asset_id).try_read().is_none(), Errors::TokenAlreadyDeployed);
        // require(caller == Identity::Address(OWNER), Errors::AccessDenied);
        storage.decimals.insert(asset_id, decimals);
        storage.name.insert(asset_id, name);
        storage.admin.insert(asset_id, caller);
        storage.total_supply.insert(asset_id, 0);
    }
}
