#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{AccountId, Environment};
use ink_lang as ink;
use ink_prelude::vec::Vec;

#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RCErrorCode {
    Failed,
    CollectionNotCreated,
    CollectionAlreadyCreated
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
    fn mint_nft(
        contract_address: AccountId,
        owner: AccountId,
        collection_id: CollectionId,
        metadata: Vec<u8>
    ) -> Result<NftId, RCError>;

    #[ink(extension = 3)]
    fn create_collection(contract_address: AccountId, metadata: Vec<u8>, symbol: Vec<u8>) -> Result<CollectionId, RCError>;
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
    use super::{RCError, RCErrorCode};

    #[ink(storage)]
    pub struct RmrkExtension {
        collection_id: Option<u32>,
    }

    impl RmrkExtension {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {
                collection_id: None,
            }
        }

        #[ink(message)]
        pub fn read_nft(&self, collection_id: u32, nft_id: u32) -> bool {
            let caller = self.env().caller();
            self.env()
                .extension()
                .read_nft(caller, collection_id, nft_id)
                .map_err(|_| false)
                .unwrap()
        }

        #[ink(message)]
        pub fn read_collection_id(&self) -> Option<u32> {
            self.collection_id
        }

        #[ink(message)]
        pub fn read_nft_id(&self) -> Option<u32> {
            self.collection_id  
        }

        #[ink(message, payable)]
        pub fn mint_nft(&mut self, metadata: ink_prelude::string::String) -> Result<u32, RCError> {
            if self.collection_id == None {
                return Err(RCError::ErrorCode(RCErrorCode::CollectionNotCreated))
            }
            let caller = self.env().caller();
            self.env()
                .extension()
                .mint_nft(self.env().account_id(), caller, self.collection_id.unwrap(), metadata.into_bytes())
        }

        #[ink(message, payable)]
        pub fn create_collection(
            &mut self,
            metadata: ink_prelude::string::String,
            symbol: ink_prelude::string::String,
        ) -> Result<u32, RCError> {
            if self.collection_id != None {
                return Err(RCError::ErrorCode(RCErrorCode::CollectionAlreadyCreated))
            }

            let result: Result<u32, RCError> = self.env()
                .extension()
                .create_collection(self.env().account_id(), metadata.into_bytes(), symbol.into_bytes());

            match result {
                Ok(cid) => self.collection_id = Some(cid),
                Err(_) => ()
            }

            return result
        }
    }
}
