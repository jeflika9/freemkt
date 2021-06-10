#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ::ink_env::{
    DefaultEnvironment,
    call::{build_create, Selector, ExecutionInput, FromAccountId}
};

#[ink::contract]
mod mod2 {
    use super::*;
    #[ink(storage)]
    pub struct Mod2 {
        // acct_id: AccountId,
    }

    impl Mod2 {
        /// Instantiate a delegator with the given sub-contract codes.
        #[ink(constructor)]
        pub fn new(
        ) -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn msg(&self) -> u32 {
            2
        }
    }

    /// # struct MyContract;
    impl FromAccountId<DefaultEnvironment> for Mod2 {
        fn from_account_id(_acct_id: AccountId) -> Self { 
            Self {}
        }
    }
}

#[ink::contract]
mod mod1 {
    use super::*;
    use mod2::Mod2;

    #[ink(storage)]
    pub struct Mod1 {}

    impl Mod1 {
        /// Instantiate a delegator with the given sub-contract codes.
        #[ink(constructor)]
        pub fn new(version: u32, mod2_code_hash: Hash) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let _mod2: Mod2 = build_create::<DefaultEnvironment, Mod2>()
                // .code_hash(Hash::from([0x42; 32]))
                .code_hash(mod2_code_hash)
                .gas_limit(10_000)
                // .endowment(25)
                .endowment(total_balance / 4)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF]))
                        .push_arg(42)
                        .push_arg(true)
                        .push_arg(&[0x10u8; 32])
                )
                .salt_bytes(salt)
                .params()
                .instantiate()
                .expect("failed at instantiating the `Mod2` contract");
            Self {}
        }

        #[ink(message)]
        pub fn msg(&self) -> u32 {
            1
        }
    }
}
