#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{Environment, AccountId};
use ink_lang as ink;

#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RCErrorCode {
    Failed,
}

#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RCError {
    ErrorCode(RCErrorCode),
}

type CollectionId = u32;
type NftId = u32;
#[ink::chain_extension]
pub trait RmrkExt {
    type ErrorCode = RCErrorCode;

    #[ink(extension = 1, returns_result = false)]
    fn read_nft(caller_id: AccountId, collection_id: CollectionId, nft_id: NftId) -> bool;

    #[ink(extension = 2)]
    fn mint_nft(contract_address: AccountId, owner: AccountId, collection_id: CollectionId) -> Result<NftId, RCError>;

    #[ink(extension = 3)]
    fn create_collection(contract_address: AccountId) -> Result<CollectionId, RCError>;
}

impl From<RCErrorCode> for RCError {
    fn from(error_code: RCErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for RCError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for RCErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = RmrkExt;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod rmrk_extension {
    use super::{RCError};

    use ink_prelude::vec::Vec;
    #[ink(storage)]
    pub struct RmrkExtension {}

    impl RmrkExtension {
        #[ink(constructor, payable)]
        pub fn new() -> Self { Self {} }


        #[ink(message)]
        pub fn read_nft(&self, collection_id: u32, nft_id: u32) -> bool {
            let caller = self.env().caller();
            self.env().extension().read_nft(caller, collection_id, nft_id).map_err(|_| false).unwrap()
        }

        #[ink(message)]
        pub fn mint_nft(&mut self, collection_id: u32, _metadata: Vec<u8>) -> Result<u32, RCError> {
            let caller = self.env().caller();
            self.env().extension().mint_nft(self.env().account_id(), caller, collection_id)
        }

        #[ink(message)]
        pub fn create_collection(&mut self, _metadata: Vec<u8>, _symbol: Vec<u8>) -> Result<u32, RCError> {
            self.env().extension().create_collection(self.env().account_id())
        }
    }
}
