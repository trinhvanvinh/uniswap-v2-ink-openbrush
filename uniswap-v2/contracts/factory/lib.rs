#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use core::hash::Hash;

use ink::primitives::AccountId;
use openbrush::traits::ZERO_ADDRESS;

#[openbrush::contract]
pub mod factory {
    use openbrush::traits::{
        Storage,
        ZERO_ADDRESS,
    };

    use uniswap_v2::{
        impls::factory::*,
        traits::factory::{
            FactoryError,
            *,
        },
    };

    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        ToAccountId,
    };

    use pair_contract::pair::PairContractRef;

    #[ink(event)]
    pub struct PairCreated {
        #[ink(topic)]
        pub token_0: AccountId,
        #[ink(topic)]
        pub token_1: AccountId,
        pub pair: AccountId,
        pub pair_len: u64,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {
        fn _emit_create_pair_event(
            &self,
            token_0: AccountId,
            token_1: AccountId,
            pair: AccountId,
            pair_len: u64,
        ) {
            EmitEvent::<FactoryContract>::emit_event(
                self.env(),
                PairCreated {
                    token_0,
                    token_1,
                    pair,
                    pair_len,
                },
            )
        }

        fn _instantiate_pair(&mut self, salt_bytes: &[u8]) -> Result<AccountId, FactoryError> {
            let pair_hash = self.factory.pair_contract_code_hash;
            let pair = match PairContractRef::new()
                .endowment(0)
                .code_hash(pair_contract)
                .salt_bytes(&salt_bytes[..4])
                .try_instantiate
            {
                Ok(Ok(res)) => Ok(res),
                _ => Err(FactoryError::PairInstantiationFailed),
            };
            Ok(pair.to_account_id())
        }
    }
}

impl FactoryContract {
    #[ink(constructor)]
    pub fn new(fee_to_setter: AccountId, pair_code_hash: Hash) -> Self {
        let mut instance = Self::default();
        instance.factory.pair_contract_code_hash = pair_code_hash;
        instance.factory.fee_to_setter = fee_to_setter;
        instance.factory.fee_to = ZERO_ADDRESS.into();
        instance
    }
}
