use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*, ActorId};
use scale_info::TypeInfo;


#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    Minted {
        to: ActorId,
        amount: u128,
    },
    BalanceOf(u128),
}