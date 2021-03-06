use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KrakenInput, MethodType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Request builder for the Cancel Open Order endpoint
pub struct KICancelOrder {
    params: IndexMap<String, String>,
}

impl KICancelOrder {
    /// Constructor returning a [KrakenInput] builder for the cancel open order endpoint.
    /// txid is the transaction ID of the order that is to be cancelled
    pub fn build(txid: String) -> KICancelOrder {
        let cancelorder = KICancelOrder {
            params: IndexMap::new(),
        };
        cancelorder.with_txid(txid)
    }

    /// Update the transaction ID of the order to cancel. Useful for templating or iterating over a
    /// list of transaction IDs without allocation
    pub fn with_txid(self, txid: String) -> Self {
        self.update_input("txid", txid)
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KICancelOrder {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelOrder {}

impl Input for KICancelOrder {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("CancelOrder"),
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
                    endpoint: String::from("CancelOrder"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

/// Response from the Cancel Open Orders endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOCancelOrder {
    /// number of orders canceled
    pub count: u32,
    /// if set, order(s) is/are pending cancellation
    pub pending: u32,
}

impl Output for KOCancelOrder {}
