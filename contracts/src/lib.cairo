/// Smart contract for converting felt252 to usize, used for Brother Yield
#[starknet::interface]
pub trait IBrotherConvert<TContractState> {
    /// Convert felt252 to usize
    fn felt252_to_usize(ref self: TContractState, value: felt252) -> usize;
    /// brudda
    fn get_brother(self: @TContractState) -> felt252;
}

/// Simple contract for managing balance.
#[starknet::contract]
mod HelloStarknet {
    use core::starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};

    #[storage]
    struct Storage {
        brother: felt252,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.brother.write('Brother');
    }

    #[abi(embed_v0)]
    impl BrotherConvertImpl of super::IBrotherConvert<ContractState> {
        fn felt252_to_usize(ref self: ContractState, value: felt252) -> usize {
            value.try_into().expect('Brother converting errror')
        }

        fn get_brother(self: @ContractState) -> felt252 {
            self.brother.read()
        }
    }
}
