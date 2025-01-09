use starknet::ContractAddress;
use snforge_std::{declare, ContractClassTrait, DeclareResultTrait};
use contracts::IBrotherConvertDispatcher;
use contracts::IBrotherConvertDispatcherTrait;

fn deploy_contract(name: ByteArray) -> ContractAddress {
    let contract = declare(name).unwrap().contract_class();
    let (contract_address, _) = contract.deploy(@ArrayTrait::new()).unwrap();
    contract_address
}

#[test]
fn test_felt252_to_usize_conversion() {
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IBrotherConvertDispatcher { contract_address };
    
    // Test small number conversion
    let small_felt: felt252 = 42;
    let small_result = dispatcher.felt252_to_usize(small_felt);
    assert(small_result == 42_usize, 'Invalid small conversion');
    
    // Test larger number conversion
    let large_felt: felt252 = 1000000;
    let large_result = dispatcher.felt252_to_usize(large_felt);
    assert(large_result == 1000000_usize, 'Invalid large conversion');
}

#[test]
fn test_get_brother() {
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IBrotherConvertDispatcher { contract_address };
    
    let brother = dispatcher.get_brother();
    assert(brother == 'Brother', 'No brother. tragic');
}

#[test]
#[should_panic(expected: 'Brother converting errror')]
fn test_felt252_to_usize_overflow() {
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IBrotherConvertDispatcher { contract_address };
    
    // Try to convert a very large felt252 that won't fit in usize
    let huge_felt: felt252 = 0x800000000000000000000000000000000000000000000000000000000000000;
    dispatcher.felt252_to_usize(huge_felt);
}
