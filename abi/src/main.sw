library;
use string::String;

abi FRC20 {
    #[storage(read)]
    fn total_supply() -> u64;
    fn decimals() -> u8;
    fn name() -> str[32];
    fn symbol() -> str[8];
    
    #[storage(read, write)]
    fn _mint(amount: u64, recipient: Address);
}