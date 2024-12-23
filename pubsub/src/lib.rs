//! `DiMAS` ppubsub example
//! Copyright © 2024 Stephan Kunz

// region:		--- modules
use dimas::prelude::*;
// endregion:	--- modules

/// common structure for publisher and subscriber
#[derive(Debug, Encode, Decode)]
pub struct PubSubMessage {
    /// counter
    pub count: u128,
    /// text
    pub text: String,
}
