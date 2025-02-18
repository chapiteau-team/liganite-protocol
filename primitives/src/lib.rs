#![cfg_attr(not(feature = "std"), no_std)]

pub mod testing;
pub mod types;

pub const MAX_NAME_SIZE: u32 = 128;
pub const MAX_URL_SIZE: u32 = 128;
