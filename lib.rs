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

    use ink::{
        env::{
            call::{
                build_call,
                ExecutionInput,
                Selector,
            },
            CallFlags,
            DefaultEnvironment,
        },
        storage::{
            Lazy,
        },
    };

    #[ink(storage)]
    pub struct CrossContractFlipper {
        value: bool,
        delegate_to: Lazy<Hash>,
    }

    impl CrossContractFlipper {
        /// Initializes a contract by specifying the code hash of the other contract
        #[ink(constructor)]
        pub fn new(init_value: bool, code_hash: Hash) -> Self {

            // Initialize the hash of the contract to delegate to.
            // Adds a delegate dependency lock, ensuring that the delegated to code cannot
            // be removed.
            let mut delegate_to = Lazy::new();
            delegate_to.set(&code_hash);

            Self::env().lock_delegate_dependency(&code_hash);

            Self {
                value: init_value,
                delegate_to
            }
        }

        /// Calls `get` method of the other contract
        /// without specifying the weight and storage limits
        #[ink(message)]
        pub fn get_value(&mut self) -> bool {
            self.value
        }

        // Delegate calls `flip` method of the other contract
        #[ink(message)]
        pub fn inc_delegate(&mut self) {
            let selector = ink::selector_bytes!("flip");
            let _ = build_call::<DefaultEnvironment>()
                .delegate(self.delegate_to())
                // We specify `CallFlags::TAIL_CALL` to use the delegatee last memory frame
                // as the end of the execution cycle.
                // So any mutations to `Packed` types, made by delegatee,
                // will be flushed to storage.
                //
                // If we don't specify this flag.
                // The storage state before the delegate call will be flushed to storage instead.
                // See https://substrate.stackexchange.com/questions/3336/i-found-set-allow-reentry-may-have-some-problems/3352#3352
                .call_flags(CallFlags::TAIL_CALL)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<()>()
                .try_invoke();
        }

        fn delegate_to(&self) -> Hash {
            self.delegate_to
                .get()
                .expect("delegate_to always has a value")
        }
    }
}
