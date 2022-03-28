#![no_std]

// External packages (crates) and internal modules import
use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*, ActorId};
use scale_info::TypeInfo;

use token::Token;
use event::Event;

pub mod token;
pub mod event;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConfig {
    name: String,
    symbol: String,
}

// Incoming messages
#[derive(TypeInfo, Decode)]
pub enum Action {
    Mint(u128),
    BalanceOf(ActorId),
}

gstd::metadata! {
    title: "token_test",
        init:
            input: InitConfig,
        handle:
            input: Action,
            output: Event,
}


static mut TOKEN: Option<Token> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitConfig= msg::load()
        .expect("Unable to decode InitConfig");

    let token = Token {
        name: config.name,
        symbol: config.symbol,
        balances: BTreeMap::new(),
    };

    TOKEN = Some(token);
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load()
        .expect("Could not load Action");

    let token: &mut Token = TOKEN.get_or_insert(Token::default());
    match action {
        Action::Mint(amount) => {
            token.mint(amount);
        }
        Action::BalanceOf(account) => {
            token.balance_of(&account);
        }
    }
}