#![no_std]

// External packages (crates) and internal modules import
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

// Incoming messages
#[derive(TypeInfo, Encode, Decode)]
pub enum Action {
    Mint(u128),
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct AddBalance {
   account: ActorId,
   token_id: ActorId,
}
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BalanceAdded {
  account: ActorId,
  token_id: ActorId,
  amount: u128,
}

gstd::metadata! {
   title: "wallet_test",
       handle:
           input: AddBalance,
           output: BalanceAdded,
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

#[gstd::async_main]
async fn main() {
    let mut wallet: BTreeMap<ActorId, BTreeMap<ActorId,u128>> = BTreeMap::default();

    let msg: AddBalance = msg::load()
        .expect("Failed to decode `AddBalance`");

    let reply: Event = msg::send_and_wait_for_reply(
        msg.token_id,
        Action::BalanceOf(msg.account),
        0
    ).await
        .expect("Function call error");

    gstd::debug!(reply);

    if let Event::BalanceOf(amount) = reply {
        wallet.entry(msg.account)
                .and_modify(|id| *id.entry(msg.token_id)
                .or_insert(0) += amount)
                .or_insert_with(|| {
                        let mut actor = BTreeMap::new();
                        actor.insert(msg.token_id, amount);
                
                        actor
                    }
                );

        msg::reply(
            BalanceAdded {
                account: msg.account,
                token_id: msg.token_id,
                amount,
            },
            0
        );
    }
}