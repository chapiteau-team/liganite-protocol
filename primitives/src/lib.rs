#![cfg_attr(not(feature = "std"), no_std)]

pub mod publisher;
pub mod tags;
pub mod testing;
pub mod types;
pub mod validate;

pub const MAX_CID_SIZE: u32 = 128;
pub const MAX_NAME_SIZE: u32 = 128;
pub const MAX_TAGS_PER_GAME: u32 = 20;
pub const MAX_TAG_SIZE: u32 = 50;
pub const MAX_URL_SIZE: u32 = 128;
