use serde::{Deserialize, Serialize};

use crate::types::VehicleState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Input { left: bool, right: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Joined {
        id: u32,
    },
    State {
        tick: u64,
        vehicles: Vec<VehicleState>,
    },
    PlayerLeft {
        id: u32,
    },
}
