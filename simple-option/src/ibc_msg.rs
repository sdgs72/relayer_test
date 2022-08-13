use cosmwasm_std::{ContractResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Messages that will be sent over the IBC channel
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PacketMsg {
    TestMsg {
        val: String,
    },
    WhoAmI { 
        chain_id: u32,
        
    },
    // TimeoutMsg{
    //     time_view: u32
    // },
}