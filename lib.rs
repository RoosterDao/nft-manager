#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{Environment, AccountId};
use ink_lang as ink;
use scale::{Decode, Encode};

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

#[ink::chain_extension]
pub trait RmrkExt {
    type ErrorCode = RCErrorCode;

    #[ink(extension = 1201)]
    fn read_nft(nft_id: u32) -> Result<AccountId, RCError>;
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

    #[ink(storage)]
    pub struct RmrkExtension {}

    impl RmrkExtension {
        #[ink(constructor)]
        pub fn new() -> Self { Self {} }


        /// Calls current_era() in the pallet-dapps-staking
        #[ink(message)]
        pub fn read_nft(&self, nft_id: u32) -> Result<AccountId, RCError> {
            self.env().extension().read_nft(nft_id)
        }
    }
}
