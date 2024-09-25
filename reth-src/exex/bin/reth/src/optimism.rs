#![allow(missing_docs, rustdoc::missing_crate_level_docs)]

// We use a global allocator for performance reasons.
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(not(feature = "optimism"))]
compile_error!("Cannot build the `op-reth` binary with the `optimism` feature flag disabled. Did you mean to build `reth`?");

use clap::Parser;
use eyre::Result;

use reth_node_optimism::{args::RollupArgs, OptimismNode};

use exex::plugin::ExExPlugin;

mod exex_implementation;
use exex_implementation::MinimalExEx;

/// You also can derive it like:
/// ```no_run
/// let exex_id = MinimalExEx::ID;
/// let exex = |ctx| MinimalExEx::exex_init(ctx);
/// ```
/// see [`MinimalExEx`].
#[cfg(feature = "optimism")]
fn main() -> Result<()> {
    // You also can derive it like:
    // let exex_id = MinimalExEx::ID;
    // let exex = |ctx| MinimalExEx::exex_init(ctx);

    let exex = MinimalExEx;
    let exex_id = exex.id();
    let exex_plugin = |ctx| exex.install_init(ctx, false);

    reth::cli::Cli::<RollupArgs>::parse().run(|builder, rollup_args| async move {
        let handle = builder
            .node(OptimismNode::new(rollup_args))
            .install_exex(exex_id, exex_plugin)
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
