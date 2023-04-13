#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {
    use openbrush::{
        contracts::psp22::{
            Internal,
            *,
        },
        traits::{
            Balance,
            Storage,
        },
    };

    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        prelude::vec::Vec,
        primitives::AccountId,
    };

    use uniswap_v2::{
        impls::pair::*,
        traits::pair::*,
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount_0: Balance,
        pub amount_1: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount_0: Balance,
        pub amount_1: Balance,
        #[ink(topic)]
        pub to: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PairContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pair: data::Data,
    }

    impl Pair for PairContract {
        fn _emit_mint_event(&self, sender: AccountId, amount_0: Balance, amount_1: Balance) {
            self.env().emit_event(Mint {
                sender,
                amount_0,
                amount_1,
            })
        }

        fn _emit_burn_event(
            &self,
            sender: AccountId,
            amount_0: Balance,
            amount_1: Balance,
            to: AccountId,
        ) {
            self.env().emit_event(Burn {
                sender,
                amount_0,
                amount_1,
                to,
            })
        }
    }

    impl PSP22 for PairContract {
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let allowance = self._allowance(from, caller);

            if allowance != u128::MAX {
                if allowance < value {
                    return Err(PSP22Error::InsufficientAllowance)
                }
                self._approve_from_to(from, caller, allowance - value);
            }

            self._transfer_from_to(from, to, value, data);
            Ok(())
        }
    }

    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();
            instance
        }
    }

    impl Internal for PairContract {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            })
        }

        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            let mut new_balance = self._balance_of(&account);
            new_balance += amount;
            self.psp22.balances.insert(&account, &new_balance);
            self.psp22.supply += amount;
            self._emit_transfer_event(None, Some(account), amount);
            Ok(())
        }

        fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            let mut from_balance = self._balance_of(&account);
            if from_balance < amount {
                return Err(PSP22Error::InsufficientBalance)
            }
            from_balance -= amount;
            self.psp22.balances.insert(&account, &from_balance);
            self.psp22.supply -= amount;
            self._emit_transfer_event(Some(account), None, amount);
            Ok(())
        }

        fn _approve_from_to(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            self.psp22.allowances.insert(&(&owner, &spender), &amount);
            self._emit_approval_event(owner, spender, amount);
            Ok(())
        }

        fn _transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let from_balance = self._balance_of(&from);

            if from_balance < amount {
                return Err(PSP22Error::InsufficientBalance)
            }

            let new_balance_from = from_balance - amount;

            self.psp22.balances.insert(&from, &from_balance);
            let to_balance = self._balance_of(&to);

            let new_balance_to = to_balance + amount;

            self.psp22.balances.insert(&to, &new_balance_to);

            self._emit_transfer_event(Some(from), Some(to), amount);

            Ok(())
        }
    }
}
