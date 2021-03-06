use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KAssetPair, KrakenInput, MethodType};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, Output, UpdateInput};

/// Request builder for the Get Trade Volume endpoint
pub struct KITradeVolume {
    params: IndexMap<String, String>,
}

impl KITradeVolume {
    /// Constructor returning a [KrakenInput] builder for the get trade volume endpoint.
    pub fn build() -> Self {
        KITradeVolume {
            params: IndexMap::new(),
        }
    }

    /// An asset pair is not required for the get trade volume endpoint. 
    /// This method clears a currently populated asset pair list.
    /// Useful for templating
    pub fn clear_pair_list(self) -> Self {
        self.update_input("pair", String::from(""))
    }

    /// Update the list of asset pairs to query fee info for
    pub fn with_pair(self, pair: KAssetPair) -> Self {
        self.with_item(pair)
    }

    /// Update the list of asset pairs to query fee info for
    pub fn with_pair_list<T>(self, pairs: T) -> Self
    where
        T: IntoIterator<Item = KAssetPair>,
    {
        self.with_item_list(pairs)
    }

    /// Should fee info be included in the results?
    pub fn with_fee_info(self, feeinfo: bool) -> Self {
        self.update_input("fee-info", feeinfo.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KITradeVolume {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeVolume {}

impl IntoInputList for KITradeVolume {
    fn list_name(&self) -> String {
        String::from("pair")
    }
}

impl InputListItem for KITradeVolume {
    type ListItem = KAssetPair;
}

impl InputList for KITradeVolume {}

impl Input for KITradeVolume {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("TradeVolume"),
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
                    endpoint: String::from("TradeVolume"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

/// Fee info
#[derive(Deserialize, Serialize, Debug)]
pub struct KOFeeInfo {
    pub fee: String,
    pub minfee: Option<String>,
    pub maxfee: Option<String>,
    pub nextfee: Option<String>,
    pub nextvolume: Option<String>,
}

/// Maker fee info
#[derive(Deserialize, Serialize, Debug)]
pub struct KOMakerFeeInfo {
    pub fee: String,
    pub minfee: Option<String>,
    pub maxfee: Option<String>,
    pub nextfee: Option<String>,
    pub nextvolume: Option<String>,
    pub tiervolume: Option<String>,
}

/// Response from the Get Trade Volume endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeVolume {
    pub currency: String,
    pub volume: String,
    pub fees: Option<HashMap<String, KOFeeInfo>>,
    pub fees_maker: Option<HashMap<String, KOMakerFeeInfo>>,
}

impl Output for KOTradeVolume {}
