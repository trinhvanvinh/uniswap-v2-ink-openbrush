#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {
    use openbrush::{
        contracts::psp22::{
            Internal,
            *,
        },
        traits::Storage,
    };

    use ink::codegen::{
        EmitEvent,
        Env,
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PairContract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for PairContract {}

    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp22: Default::default(),
            }
        }
    }
}
