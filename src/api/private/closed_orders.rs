use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType, OrderCloseTime};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

pub use super::KOOrderDescription;
pub use super::KOOrderInfo;
pub use super::KOOrderStatus;

/// Request builder for the Get Closed Orders endpoint
pub struct KIClosedOrders {
    params: IndexMap<String, String>,
}

impl KIClosedOrders {
    /// Constructor returning a [KrakenInput] builder for the get closed orders endpoint.
    pub fn build() -> Self {
        KIClosedOrders {
            params: IndexMap::new(),
        }
    }

    /// Should trades be included in returned output?
    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    /// Filter results to the given user ref id. 
    /// A custom userref can be passed into the add order endpoint
    pub fn with_userref(self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    /// Starting Unix timestamp to filter output by. Exclusive
    pub fn starting_timestamp(self, timestamp: u64) -> Self {
        self.update_input("start", timestamp.to_string())
    }

    /// Ending Unix timestamp to filter output by. Inclusive
    pub fn ending_timestamp(self, timestamp: u64) -> Self {
        self.update_input("end", timestamp.to_string())
    }

    /// Starting transaction ID to filter output by. Exclusive
    pub fn starting_txid(self, txid: String) -> Self {
        self.update_input("start", txid)
    }

    /// Ending transaction ID to filter output by. Inclusive
    pub fn ending_txid(self, txid: String) -> Self {
        self.update_input("end", txid)
    }

    /// Result offset. Not clear what this does
    pub fn with_offset(self, offset: u64) -> Self {
        self.update_input("ofs", offset.to_string())
    }

    /// Query orders by open time, close time, or both
    pub fn with_closetime(self, closetime: OrderCloseTime) -> Self {
        self.update_input("closetime", closetime.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIClosedOrders {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("ClosedOrders"),
            },
            params: Some(self.with_nonce().params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        let newself = self.with_nonce();
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Private,
                    endpoint: String::from("ClosedOrders"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

impl MutateInput for KIClosedOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIClosedOrders {}

/// Response from the Get Closed Orders endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOClosedOrders {
    pub closed: HashMap<String, KOOrderInfo>,
    pub count: u32,
}

impl Output for KOClosedOrders {}
