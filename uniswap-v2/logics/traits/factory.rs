use openbrush::traits::AccountId;

use super::pair::PairError;

#[openbrush::wrapper]
pub type FactoryRef = dyn Factory;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn fee_to(&self) -> AccountId;

    #[ink(message)]
    fn all_pair_length(&self) -> u64;

    #[ink(message)]
    fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), FactoryError>;

    #[ink(message)]
    fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), FactoryError>;

    #[ink(message)]
    fn fee_to_setter(&self) -> AccountId;

    #[ink(message)]
    fn create_pair(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, FactoryError>;

    #[ink(message)]
    fn get_pair(&self, token_a: AccountId, token_b: AccountId) -> Option<AccountId>;

    fn _emit_create_pair_event(
        &self,
        _token_0: AccountId,
        _token_1: AccountId,
        _apair: AccountId,
        _apair_len: u64,
    );

    fn _instantiate_pair(&mut self, salt_bytes: &[u8]) -> Result<AccountId, FactoryError> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FactoryError {
    PairError(PairError),
    CallerIsnotFeeSetter,
    ZeroAddress,
    IdenticalAddress,
    PairInstantiationFailed,
}

impl From<PairError> for FactoryError {
    fn from(value: PairError) -> Self {
        FactoryError::PairError(value)
    }
}
