#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![allow(missing_docs)]

#[cfg(all(feature = "optimism", not(test)))]
compile_error!("Cannot build the `reth` binary with the `optimism` feature flag enabled. Did you mean to build `op-reth`?");

use eyre::Result;

#[cfg(not(feature = "optimism"))]
fn main() -> Result<()> {
    todo!()
}
