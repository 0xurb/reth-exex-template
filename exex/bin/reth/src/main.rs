#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![allow(missing_docs)]

// We use a global allocator for performance reasons.
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "optimism", not(test)))]
compile_error!("Cannot build the `reth` binary with the `optimism` feature flag enabled. Did you mean to build `op-reth`?");

mod exex_implementation;
use exex_implementation::MinimalExEx;

use eyre::Result;

use reth_node_ethereum::EthereumNode;

use exex::plugin::ExExPlugin;

/// You also can derive it like:
/// ```no_run
/// let exex_id = MinimalExEx::ID;
/// let exex = |ctx| MinimalExEx::exex_init(ctx);
/// ```
/// see [`MinimalExEx`].
#[cfg(not(feature = "optimism"))]
fn main() -> Result<()> {
    // You also can derive it like:
    // let exex_id = MinimalExEx::ID;
    // let exex = |ctx| MinimalExEx::exex_init(ctx);

    let exex = MinimalExEx::default();
    let exex_id = exex.id();
    let exex_plugin = |ctx| exex.install_init(ctx, false);

    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex(exex_id, exex_plugin)
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
