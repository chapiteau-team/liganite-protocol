use crate::{MAX_NAME_SIZE, MAX_URL_SIZE};
use frame_support::pallet_prelude::*;

pub type Name = BoundedVec<u8, ConstU32<MAX_NAME_SIZE>>;
pub type Url = BoundedVec<u8, ConstU32<MAX_URL_SIZE>>;
