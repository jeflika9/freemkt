#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
pub mod cross_callee {
    /// Storage for the other contract.
    #[ink(storage)]
    pub struct CrossCallee {
        pub value: i32,
    }

    impl CrossCallee {
        /// Initializes the contract.
        #[ink(constructor)]
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        /// Returns the current state.
        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            self.value
        }
    }
}