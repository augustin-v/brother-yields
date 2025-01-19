
#[starknet::interface]
pub trait IBrotherYieldPass<TContractState> {
    fn mint(ref self: TContractState);
    fn get_holders_num(self: @TContractState) -> u32;
    fn check_minted(ref self: TContractState);
}