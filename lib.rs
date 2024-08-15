#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Docs on utilities to call or instantiate contracts on the chain:
/// https://docs.rs/ink_env/5.0.0/ink_env/call/index.html

#[ink::contract]
mod cross_contract_flipper {
    use ink::codegen::TraitCallBuilder;
    use other_contract::OtherContractRef;
    use ink::env::{
        call::{build_create, build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

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
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_contract, other_contract_code_hash }
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
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(true)
                )
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .returns::<OtherContractRef>()
                .instantiate();

            Self { other_contract, other_contract_code_hash }
        }

        /// Calls the `flip` method of the other contract
        /// using the call builder API
        /// https://docs.rs/ink_env/5.0.0/ink_env/call/struct.CallBuilder.html
        #[ink(message)]
        pub fn flip_usig_builder(&mut self) {
            build_call::<DefaultEnvironment>()
                .call_v1(self.get_other_contract_account_id())
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
                )
                .returns::<()>()
                .invoke();
        }

        /// Calls `flip` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn flip(&mut self) {
            self.other_contract.flip();
        }

        /// Calls `set` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn set(&mut self) {
            self.other_contract.set(true);
        }

        /// Calls `get` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn get(&mut self) -> bool {
            self.other_contract.get()
        }

        /// Calls the `flip` method of the other contract
        /// with the specified weight and storage limits
        /// using the call builder API.
        #[ink(message)]
        pub fn flip_with_limits(
            &mut self,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) {
            let call_builder = self.other_contract.call_mut();

            call_builder
                .flip()
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .invoke();
        }

        /// Calls the `get` method of the other contract
        /// with the specified weight and storage limits
        /// using the call builder API.
        #[ink(message)]
        pub fn get_with_limits(
            &mut self,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) -> bool {
            let call_builder = self.other_contract.call_mut();

            call_builder
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
        #[ink(message)]
        pub fn delegate_flip(&mut self) {
            let _ = build_call::<DefaultEnvironment>()
                .delegate(self.other_contract_code_hash)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!("flip"))))
                .returns::<()>()
                .try_invoke();
        }
        
    }
}
