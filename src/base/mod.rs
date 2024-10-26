mod cfg;
mod err;
mod net;

use std::env::set_var;

pub use cfg::*;
pub use err::*;
pub use net::*;

pub use clap::Parser;
pub use crossbeam_channel::*;
pub use log::{debug, error, info, trace, warn};
pub use packet_enum::*;
pub use parking_lot::*;
pub use serde::{Deserialize, Serialize};
pub use strum::Display;
pub use thiserror::{self, Error as ThisError};

pub fn init_logger() {
    set_var("RUST_LOG", "trace");
    env_logger::init();
}
