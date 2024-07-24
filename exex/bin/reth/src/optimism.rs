use eyre::Result;

#[cfg(not(feature = "optimism"))]
compile_error!("Cannot build the `op-reth` binary with the `optimism` feature flag disabled. Did you mean to build `reth`?");

#[cfg(feature = "optimism")]
fn main() -> Result<()> {
    todo!()
}
