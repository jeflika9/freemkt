#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// To make cross-calling compile correctly, need to:
//   A. Declare in callee's Cargo.toml [features] section:
//            ink-as-dependency = []

//   B. Declare rlib in callee's Cargo.toml [lib] section, crate-type declaration as shown here (Note: cdylib should be declared for all contracts as shown here).
//      This tells the compiler to preserve names, so that, e.g., pub mod cross_callee {...} in callee is reflected with the name "cross_callee" when the
//      crate is used
//            crate-type = [
// 	              # Used for normal contract Wasm blobs.
// 	              "cdylib",
//                # Used for ABI generation.
//                "rlib",
//            ]

//      NOTE:  The rlib file is a static file that contains the library's metadata that allows the compiler to link the library into other libraries 
//             (i.e., those that import the library as a dependency).

//   C. Import the callee as a dependency such as here:
//            cross_callee = { path = "../cross_callee", version = "0.1.0", default-features = false, features = ["ink-as-dependency"] }

//   D. Add a "use" statement such as below, just as if the library were a "normal" library rather than an ink dependency:
//            use cross_callee::cross_callee::CrossCallee;

//   E. Create a newly-instantiated instance of the contract via its upload hash such as below (this uses the CreateBuilder type in ink!):
//            let crosscallee = CrossCallee::new(1337)
//                                .endowment(25)
//                                .code_hash(cross_callee_code_hash)
//                                // .gas_limit(10_000) // this setting is optional (presumably the called contracts' gas limits would apply if not limited here?)
//                                .salt_bytes(1u32.to_le_bytes())
//                                .instantiate()
//                                .expect("failed at instantiating the `CrossCallee` contract");

//   F. Call the contract's API via the newly-instantiated instance, such as:
//            self.crosscallee.get_value()


#[ink::contract]
mod mod2 {
    use ink_storage::Lazy;
    use cross_callee::cross_callee::CrossCallee;
    //--snip--
    #[ink(storage)]
    pub struct MyContract {
        /// The other contract.
        crosscallee: Lazy<CrossCallee>,
    }

    impl MyContract {

        /// Instantiate `MyContract with the given
        /// sub-contract codes and some initial value.
        #[ink(constructor)]
        pub fn new(
            cross_callee_code_hash: Hash,
        ) -> Self {
            let crosscallee = CrossCallee::new(1337)
                .endowment(25)
                .code_hash(cross_callee_code_hash)
                // .gas_limit(10_000) // this setting is optional (presumably the called contracts' gas limits would apply if not limited here?)
                .salt_bytes(1u32.to_le_bytes())
                .instantiate()
                .expect("failed at instantiating the `CrossCallee` contract");
            Self {
                crosscallee: Lazy::new(crosscallee),
            }
        }

        /// Calls the other contract.
        #[ink(message)]
        pub fn call_other_contract(&self) -> i32 {
            self.crosscallee.get_value()
        }
    }
    //--snip--
}