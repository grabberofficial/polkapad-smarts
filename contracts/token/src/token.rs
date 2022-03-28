use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*, ActorId};
use scale_info::TypeInfo;

use crate::event::Event;

// Outcoming messages

#[derive(Clone, TypeInfo, Decode, Encode)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub balances: BTreeMap<ActorId, u128>,
}

impl Default for Token {
    fn default() -> Self {
        Token { 
            name: String::new(), 
            symbol: String::new(), 
            balances: BTreeMap::default()
         }
    }
}

impl Token {
    pub fn mint(&mut self, amount: u128) {
        let minting_actor = msg::source();

        *self.balances
            .entry(minting_actor)
            .or_insert(0) += amount;

        msg::reply(
            Event::Minted {
                to: minting_actor,
                amount
            },
            0
        );
    }

    pub fn balance_of(&mut self, account: &ActorId) {
        let balance = self.balances
            .get(account)
            .unwrap_or(&0);
        
        msg::reply(
            *balance,
            0
        );
    }
}
