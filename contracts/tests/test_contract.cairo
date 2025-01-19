use snforge_std::{declare, ContractClassTrait, DeclareResultTrait, start_cheat_caller_address};
use contracts::contract::interface::IBrotherYieldPassDispatcher;
use contracts::contract::interface::IBrotherYieldPassDispatcherTrait;
use starknet::ContractAddress;


fn deploy_contract() -> ContractAddress {
    let contract = declare("BrotherYieldPass").unwrap().contract_class();
    
    // Precalculate address for constructor cheats
    let contract_address = contract.precalculate_address(@array![123.try_into().unwrap()]);
    
    // Start cheats before deployment
    start_cheat_caller_address(contract_address, 123.try_into().unwrap());
    
    // Deploy with owner address in constructor args
    let constructor_args = array![123.try_into().unwrap()];
    let (address, _) = contract.deploy(@constructor_args).unwrap();
    address
}

#[test]
fn test_mint_pass() {
    let contract_address = deploy_contract();
    let dispatcher = IBrotherYieldPassDispatcher { contract_address };
    
    dispatcher.mint();
    assert(dispatcher.get_holders_num() == 1, 'Invalid holders count');

    dispatcher.check_minted();
}

#[test]
#[should_panic(expected: "Only mintable once")]
fn test_double_mint() {
    let contract_address = deploy_contract();
    let dispatcher = IBrotherYieldPassDispatcher { contract_address };
    
    dispatcher.mint();
    dispatcher.mint();
}

#[test]
#[should_panic]
fn check_nomint() {
    let contract_address = deploy_contract();
    let result = IBrotherYieldPassDispatcher { contract_address  };
    result.check_minted();
}