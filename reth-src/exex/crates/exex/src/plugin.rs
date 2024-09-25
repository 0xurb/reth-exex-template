use std::{fmt::Debug, future::Future};

use eyre::Result;

use reth::api::FullNodeComponents;
use reth_exex::ExExContext;
use reth_tracing::tracing::info;

/// This interface is used to simplify `ExEx` logic and split it into:
/// - `setup` - some routine before `ExEx` is launched. Can be async, can be ignored (has default
///   implementation).
/// - `exex` - main logic of `ExEx` with [`ExExContext`] as a given param and [`ExExNotification`]
///   handler from this context notification channel.
pub trait ExExPlugin<Id>: Sized
where
    Id: Into<String> + Debug,
{
    fn id(&self) -> Id;

    /// `ExEx` setup routine, if needed
    /// Can be a DB connection pool opening, state syncing, etc.
    fn setup(&self) -> impl Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    /// The `ExEx` future.
    /// Implements a logic to:
    /// - receive context notifications ([ExExNotification](`reth_exex::ExExNotification`))
    /// - process chain of blocks and their final state ([chain](`Chain`))
    /// - send events [ExExEvent](`reth_exex::ExExEvent`) to context event channel
    fn exex<N: FullNodeComponents>(
        self,
        ctx: ExExContext<N>,
    ) -> impl Future<Output = Result<()>> + Send;

    /// The initialization logic of the `ExEx` install process.
    /// Default implementation, which combines an `ExEx` setup and execution.
    fn install_init<N: FullNodeComponents>(
        self,
        ctx: ExExContext<N>,
        with_setup: bool,
    ) -> impl std::future::Future<Output = Result<impl Future<Output = Result<()>> + Send>> {
        let id = self.id();
        async move {
            if with_setup {
                self.setup().await?;
                info!(name = ?id, "ExEx is setted up");
            }

            Ok(self.exex(ctx))
        }
    }
}
