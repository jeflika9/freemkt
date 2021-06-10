#[ink::contract]
mod mod1 {
    use super::*;
    use mod2::Mod2;
    // use ink_storage::{
    //     traits::{
    //         PackedLayout,
    //         SpreadLayout,
    //     },
    //     Lazy,
    // };

    #[ink(storage)]
    pub struct Mod1 {}

    impl Mod1 {
        /// Instantiate a delegator with the given sub-contract codes.
        #[ink(constructor)]
        pub fn new(init_value: i32, version: u32, mod2_code_hash: Hash) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            type Hash = <DefaultEnvironment as Environment>::Hash;
            type AccountId = <DefaultEnvironment as Environment>::AccountId;
            type Salt = &'static [u8];

            let mod2: Mod2 = build_create::<DefaultEnvironment, Mod2>()
                .code_hash(Hash::from([0x42; 32]))
                .gas_limit(4000)
                .endowment(25)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF]))
                        .push_arg(42)
                        .push_arg(true)
                        .push_arg(&[0x10u8; 32])
                )
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .params()
                .instantiate()
                .unwrap();
            let mod25 = mod2
                .endowment(total_balance / 4)
                .code_hash(mod2_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .expect("failed at instantiating the `Accumulator` contract");
            Self {}
        }

        #[ink(message)]
        pub fn msg(&self) -> u32 {
            1
        }
    }
}
