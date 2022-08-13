use std::collections::{HashSet};

use cosmwasm_std::{IbcMsg, Timestamp, SubMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // pub role: String,
    pub n: u32,
    pub chain_id: u32,
    pub channel_ids: Vec<String>,
}


impl State {
    pub(crate) fn new(chain_id: u32, input: String, start_time: Timestamp) -> Self {
        Self {
            n: 1,
            chain_id: chain_id,
            channel_ids: Vec::new(),
        }
    }
}


pub const STATE: Item<State> = Item::new("state");
pub const CHANNELS: Map<u32, String> = Map::new("channels");

pub const DEBUG: Map<u32, String> = Map::new("debug");
