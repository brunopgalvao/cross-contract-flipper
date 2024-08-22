#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::other_contract::{
    OtherContractRef,
};

#[ink::contract]
mod other_contract {

    #[ink(storage)]
    pub struct OtherContract {
        value: bool,
    }

    impl OtherContract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            self.value = value;
        }

        // Return account_id of the contract
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }
}
