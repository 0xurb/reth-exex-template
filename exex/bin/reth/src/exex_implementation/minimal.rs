//! Minimal ExEx example ([`source`](https://github.com/paradigmxyz/reth-exex-examples/blob/main/minimal/src/main.rs))
//! [`MinimalExEx`] can be constructed as:
//!
//! 1. Simple initialization logic, which do the future with chain processing.
//! 2. [`ExExPlugin`] implementation, which possible may have a setup.

use std::future::{self, Future};

use eyre::Result;
use reth::api::FullNodeComponents;

use exex::{plugin::ExExPlugin, tracing::info, ExExContext, ExExEvent, ExExNotification};

/// Minimal ExEx
#[derive(Debug, Default)]
pub(crate) struct MinimalExEx;

#[allow(unused)]
impl MinimalExEx {
    pub(crate) const ID: &'static str = "Minimal";

    /// The initialization logic of the ExEx is just an async function.
    ///
    /// During initialization you can wait for resources you need to be up for the ExEx to function,
    /// like a database connection.
    pub(crate) async fn exex_init<Node: FullNodeComponents>(
        ctx: ExExContext<Node>,
    ) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
        Ok(MinimalExEx::exex(ctx))
    }

    /// An ExEx is just a future, which means you can implement all of it in an async function!
    ///
    /// This ExEx just prints out whenever either a new chain of blocks being added, or a chain of
    /// blocks being re-orged. After processing the chain, emits an [ExExEvent::FinishedHeight]
    /// event.
    async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
        while let Some(notification) = ctx.notifications.recv().await {
            match &notification {
                ExExNotification::ChainCommitted { new } => {
                    info!(committed_chain = ?new.range(), "Received commit");
                }
                ExExNotification::ChainReorged { old, new } => {
                    info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
                }
                ExExNotification::ChainReverted { old } => {
                    info!(reverted_chain = ?old.range(), "Received revert");
                }
            };

            if let Some(committed_chain) = notification.committed_chain() {
                ctx.events.send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
            }
        }

        Ok(())
    }
}

/// Also, alternatively you can implement the [`plugin`](`ExExPlugin`) trait for `ExEx`
impl ExExPlugin<&str> for MinimalExEx {
    #[inline(always)]
    fn id(&self) -> &'static str {
        "Minimal"
    }

    fn setup(&self) -> impl Future<Output = Result<()>> + Send {
        future::ready(Ok(()))
    }

    async fn exex<N: FullNodeComponents>(self, mut ctx: ExExContext<N>) -> Result<()> {
        while let Some(notification) = ctx.notifications.recv().await {
            match &notification {
                ExExNotification::ChainCommitted { new } => {
                    info!(committed_chain = ?new.range(), "Received commit");
                }
                ExExNotification::ChainReorged { old, new } => {
                    info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
                }
                ExExNotification::ChainReverted { old } => {
                    info!(reverted_chain = ?old.range(), "Received revert");
                }
            };

            if let Some(committed_chain) = notification.committed_chain() {
                ctx.events.send(ExExEvent::FinishedHeight(committed_chain.tip().number))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use reth_execution_types::{Chain, ExecutionOutcome};
    use reth_exex_test_utils::{test_exex_context, Adapter, PollOnce};
    use std::{future::Future, pin::pin};

    use exex::{plugin::ExExPlugin, ExExContext};

    use super::MinimalExEx;

    async fn _test_exex<ExEx, R, E>(exex: ExEx) -> eyre::Result<()>
    where
        ExEx: FnOnce(ExExContext<Adapter>) -> R + Send + 'static,
        R: Future<Output = eyre::Result<E>> + Send,
        E: Future<Output = eyre::Result<()>> + Send,
    {
        // Initialize a test Execution Extension context with all dependencies
        let (ctx, mut handle) = test_exex_context().await?;

        // Save the current head of the chain to check the finished height against it later
        let head = ctx.head;

        // Send a notification to the Execution Extension that the chain has been committed
        handle
            .send_notification_chain_committed(Chain::from_block(
                handle.genesis.clone(),
                ExecutionOutcome::default(),
                None,
            ))
            .await?;

        // Initialize the Execution Extension
        let mut exex = pin!(exex(ctx).await?);

        // Check that the Execution Extension did not emit any events until we polled it
        handle.assert_events_empty();

        // Poll the Execution Extension once to process incoming notifications
        exex.poll_once().await?;

        // Check that the Execution Extension emitted a `FinishedHeight` event with the correct
        // height
        handle.assert_event_finished_height(head.number)?;

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_exex() -> eyre::Result<()> {
        _test_exex(|ctx| MinimalExEx::exex_init(ctx)).await
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_exex_plugin() -> eyre::Result<()> {
        let exex = MinimalExEx::default();

        _test_exex(|ctx| exex.install_init(ctx, false)).await
    }
}
