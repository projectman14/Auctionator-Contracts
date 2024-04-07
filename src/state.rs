use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Auction {
    pub starting_bid: u128,
    pub current_bid: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bids {
    pub amount: u128,
}

pub const AUCTION: Item<Auction> = Item::new("auction");
pub const BIDS: Map<String, Bids> = Map::new("bids2address");
