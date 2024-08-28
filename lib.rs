// If `std` feature is disabled, we are building for Wasm target
// and we need to use `no_std` and `no_main` attributes
// to compile the contract as a Wasm binary.
// If `std` feature is enabled, we are building for native target
// and we don't need these attributes. 
// ink! builds in `std` mode when running tests.
//
// `no_std` attribute disables the standard library.
// When `no_std` is enabled, the `core` and `alloc` libraries are available.
// The `ink` crate provides necessary functionality in place of the standard library.
// `no_main` attribute disables the default entry point for the binary.
// We define our own entry point using the `#[ink::contract]` attribute.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Docs on utilities to call or instantiate contracts on the chain:
/// https://docs.rs/ink_env/5.0.0/ink_env/call/index.html

#[ink::contract]
mod cross_contract_flipper {
    /// The global call builder type for an ink! trait definition.
    /// Allows us to use call_mut() which is a method of TraitCallBuilder
    /// that returns an exclusive reference to the global call builder type.
    use ink::codegen::TraitCallBuilder;

    use ink::env::{
        call::{build_call, build_create, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct CrossContractFlipper {
        other_contract: OtherContractRef,
        other_contract_code_hash: Hash,
    }

    impl CrossContractFlipper {
        /// Initializes a contract by specifying the code hash of the other contract
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash) -> Self {
            let other_contract = OtherContractRef::new(true)
                .code_hash(other_contract_code_hash)
                // Amount transferred upon the execution of this call (instantiation).
                .endowment(0)
                // The salt for determining the hash for the contract account ID.
                // Use to create multiple instances of the same contract code from the same account.
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self {
                other_contract,
                other_contract_code_hash,
            }
        }

        /// Initializes a contract by specifying the code hash of the other contract
        /// using the create builder API
        /// https://docs.rs/ink_env/5.0.0/ink_env/call/struct.CreateBuilder.html
        #[ink(constructor)]
        pub fn new_with_create_builder(other_contract_code_hash: Hash) -> Self {
            let other_contract: OtherContractRef = build_create::<OtherContractRef>()
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new"))).push_arg(true),
                )
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .returns::<OtherContractRef>()
                .instantiate();

            Self {
                other_contract,
                other_contract_code_hash,
            }
        }

        /// Calls the `flip` method of the other contract
        /// using the call builder API
        /// https://docs.rs/ink_env/5.0.0/ink_env/call/struct.CallBuilder.html
        #[ink(message)]
        pub fn build_call_flip_1(&mut self) {
            build_call::<DefaultEnvironment>()
                .call(self.get_other_contract_account_id())
                // Amount of funds that are transferred to the other contract with this call.
                .transferred_value(0)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "flip"
                ))))
                .returns::<()>()
                .invoke();
        }

        /// Calls the `flip` method of another contract dynamically
        /// using the call builder API and specifying the account ID of the other contract
        /// https://docs.rs/ink_env/5.0.0/ink_env/call/struct.CallBuilder.html
        #[ink(message)]
        pub fn build_call_flip_2(&mut self, other_contract_account_id: AccountId) {
            build_call::<DefaultEnvironment>()
                .call(other_contract_account_id)
                .transferred_value(0)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "flip"
                ))))
                .returns::<()>()
                .invoke();
        }

        /// Calls `flip` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn call_flip(&mut self) {
            self.other_contract.flip();
        }

        /// Calls `set` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn call_set(&mut self) {
            self.other_contract.set(true);
        }

        /// Calls `get` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn call_get(&mut self) -> bool {
            self.other_contract.get()
        }

        /// Calls the `flip` method of the other contract
        /// with the specified weight and storage limits
        #[ink(message)]
        pub fn call_flip_with_limits(
            &mut self,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) {
            self.other_contract
                .call_mut()
                .flip()
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .invoke();
        }

        /// Calls the `get` method of the other contract
        /// with the specified weight and storage limits
        #[ink(message)]
        pub fn call_get_with_limits(
            &mut self,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) -> bool {
            self.other_contract
                .call_mut()
                .get()
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .invoke()
        }

        /// Calls `get_account_id` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn get_other_contract_account_id(&mut self) -> AccountId {
            self.other_contract.get_account_id()
        }

        // Delegate calls `flip` method of the other contract
        // using `DelegateCall` method of the call builder API
        // https://docs.rs/ink_env/5.0.0/ink_env/call/struct.DelegateCall.html
        // https://docs.rs/ink_env/5.0.0/ink_env/call/fn.build_call.html#example-3-delegate-call
        // https://medium.com/coinmonks/delegatecall-calling-another-contract-function-in-solidity-b579f804178c
        #[ink(message)]
        pub fn delegate_flip(&mut self) {
            // Bonus Exercise: Delegate call to the other contract
            // Submit a PR to this repo when completed
            // Include steps to test the delegate call, even better would be e2e tests ;)
            todo!()
        }
    }
}
