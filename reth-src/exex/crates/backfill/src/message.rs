use std::ops::RangeInclusive;

use tokio::sync::oneshot;

/// The message type used to communicate with the ExEx.
#[derive(Debug)]
pub enum BackfillMessage {
    /// Start a backfill job for the given range.
    ///
    /// The job ID will be sent on the provided channel.
    Start { range: RangeInclusive<u64>, response_tx: oneshot::Sender<eyre::Result<u64>> },
    /// Cancel the backfill job with the given ID.
    ///
    /// The cancellation result will be sent on the provided channel.
    Cancel { job_id: u64, response_tx: oneshot::Sender<eyre::Result<()>> },
    /// Finish the backfill job with the given ID, if it exists.
    Finish { job_id: u64 },
}
