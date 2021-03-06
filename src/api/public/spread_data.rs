use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, KAssetPair, KrakenInput, MethodType, MutateInput, Output, UpdateInput,
};

/// Request builder for the Get Recent Spread Data endpoint
pub struct KISpreadData {
    params: IndexMap<String, String>,
}

impl KISpreadData {
    /// Constructor returning a [KrakenInput] builder for the get recent spread data endpoint.
    /// * `pair` is the asset pair to query OHLC data for
    pub fn build(pair: KAssetPair) -> Self {
        let spread = KISpreadData {
            params: IndexMap::new(),
        };
        spread.update_pair(pair)
    }

    /// Update the asset pair to query OHLC data for
    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    /// Unix timestamp to query OHLC data from. The [KOSpreadData] member `last` can be used as input to
    /// `since()` to query trades data since the last time data was requested
    pub fn since(self, id: String) -> Self {
        self.update_input("since", id)
    }
}

impl Input for KISpreadData {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("Spread"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("Spread"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
    }
}

impl MutateInput for KISpreadData {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KISpreadData {}

/// Spread info
#[derive(Deserialize, Serialize, Debug)]
pub struct KOSpreadInfo {
    pub time: i64,
    pub bid: String,
    pub ask: String,
}

/// Response from the Get Recent Spread Data endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOSpreadData {
    /// Map with the asset pair as the key and the pair's Spread data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, Vec<KOSpreadInfo>>,
    /// ID to be used as "since" input to subsequent Spread Data requests
    pub last: i64,
}

impl Output for KOSpreadData {}
