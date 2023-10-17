library;

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