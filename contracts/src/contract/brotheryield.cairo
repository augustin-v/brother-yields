#[starknet::contract]
pub mod BrotherYieldPass {
    use ERC721Component::InternalTrait;
    use starknet::storage::{
        StoragePointerWriteAccess, StoragePointerReadAccess, StoragePathEntry, Map
    };
    use starknet::{ContractAddress, get_caller_address, ClassHash};
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_token::erc721::{ERC721Component, ERC721HooksEmptyImpl};
    use openzeppelin_introspection::src5::SRC5Component;
    use openzeppelin_upgrades::UpgradeableComponent;
    use openzeppelin_upgrades::interface::IUpgradeable;
    use contracts::contract::interface::IBrotherYieldPass;




    component!(path: ERC721Component, storage: erc721, event: ERC721Event);
    component!(path: SRC5Component, storage: src5, event: SRC5Event);
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradable, event: UpgradeableEvent);

    #[abi(embed_v0)]
    impl ERC721Impl = ERC721Component::ERC721Impl<ContractState>;
    impl ERC721InternalImpl = ERC721Component::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl SRC5Impl = SRC5Component::SRC5Impl<ContractState>;
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        minted: Map<ContractAddress, bool>,
        total_minted: u32,
        #[substorage(v0)]
        upgradable: UpgradeableComponent::Storage,
        #[substorage(v0)]
        pub erc721: ERC721Component::Storage,
        #[substorage(v0)]
        pub src5: SRC5Component::Storage,
        #[substorage(v0)]
        pub ownable: OwnableComponent::Storage,

    }
    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ERC721Event: ERC721Component::Event,
        #[flat]
        SRC5Event: SRC5Component::Event,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
    }
    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.erc721.initializer("BrotherYields", "BROED", "secret");
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl UpgradeableImpl of IUpgradeable<ContractState> {
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradable.upgrade(new_class_hash);
        }
    }

    #[abi(embed_v0)]
    impl BrotherYieldPassImpl of IBrotherYieldPass<ContractState>{
        fn mint(ref self: ContractState) {
            let caller = get_caller_address();

            assert!(!self.minted.entry(caller).read(), "Only mintable once");
            self.erc721.mint(caller, self.get_holders_num().into() + 1);
            self.total_minted.write(self.get_holders_num() + 1);
            self.minted.entry(caller).write(true);
        }

        fn get_holders_num(self: @ContractState) -> u32 {
            self.total_minted.read()
        }

        fn check_minted(ref self: ContractState) {
            let caller = get_caller_address();
            assert!(self.minted.entry(caller).read(), "Must mint pass to interact");
        }
    }
}