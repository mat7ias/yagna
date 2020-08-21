//! This module is to be used only in tests.
#![allow(dead_code)]
#![allow(unused_macros)]

pub use super::db::dao::*;
pub use super::db::model::*;
pub use super::matcher::{error::*, *};
pub use super::negotiation::{error::*, *};
pub use super::protocol::*;

pub mod bcast;
pub mod events_helper;
pub mod mock_net;
pub mod mock_node;
pub mod mock_offer;
pub mod proposal_util;

pub use mock_node::{wait_for_bcast, MarketServiceExt, MarketsNetwork};
pub use mock_offer::{client, generate_identity, sample_demand, sample_offer};
