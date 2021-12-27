use crate::utils::{read_atomic, set_atomic};
use lazy_static::lazy_static;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[cfg(feature = "custom-reconnect-errors")]
use parking_lot::RwLock;

/// Special errors that can trigger reconnection logic, which can also retry the failing command if possible.
///
/// `MOVED`, `ASK`, and `NOAUTH` errors are handled separately by the client.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg(feature = "custom-reconnect-errors")]
pub enum ReconnectError {
  /// The CLUSTERDOWN prefix.
  ClusterDown,
  /// The LOADING prefix.
  Loading,
  /// The MASTERDOWN prefix.
  MasterDown,
  /// The READONLY prefix, which can happen if a primary node is switched to a replica without any connection interruption.
  ReadOnly,
  /// The MISCONF prefix.
  Misconf,
  /// The BUSY prefix.
  Busy,
  /// The NOREPLICAS prefix.
  NoReplicas,
  /// A case-sensitive prefix on an error message.
  ///
  /// See [the source](https://github.com/redis/redis/blob/unstable/src/server.c#L2506-L2538) for examples.
  Custom(&'static str),
}

#[cfg(feature = "custom-reconnect-errors")]
impl ReconnectError {
  pub(crate) fn to_str(&self) -> &'static str {
    use ReconnectError::*;

    match self {
      ClusterDown => "CLUSTERDOWN",
      Loading => "LOADING",
      MasterDown => "MASTERDOWN",
      ReadOnly => "READONLY",
      Misconf => "MISCONF",
      Busy => "BUSY",
      NoReplicas => "NOREPLICAS",
      Custom(prefix) => prefix,
    }
  }
}

/// Mutable globals that can be configured by the caller.
pub(crate) struct Globals {
  /// The default timeout to apply to connections to sentinel nodes.
  pub(crate) sentinel_connection_timeout_ms: Arc<AtomicUsize>,
  #[cfg(feature = "blocking-encoding")]
  /// The minimum size, in bytes, of frames that should be encoded or decoded with a blocking task.
  pub(crate) blocking_encode_threshold: Arc<AtomicUsize>,
  /// Any special errors that should trigger reconnection logic.
  #[cfg(feature = "custom-reconnect-errors")]
  pub(crate) reconnect_errors: Arc<RwLock<Vec<ReconnectError>>>,
}

impl Default for Globals {
  fn default() -> Self {
    Globals {
      sentinel_connection_timeout_ms: Arc::new(AtomicUsize::new(200)),
      #[cfg(feature = "blocking-encoding")]
      blocking_encode_threshold: Arc::new(AtomicUsize::new(500_000)),
      #[cfg(feature = "custom-reconnect-errors")]
      reconnect_errors: Arc::new(RwLock::new(vec![
        ReconnectError::ClusterDown,
        ReconnectError::Loading,
        ReconnectError::ReadOnly,
      ])),
    }
  }
}

impl Globals {
  pub fn sentinel_connection_timeout_ms(&self) -> usize {
    read_atomic(&self.sentinel_connection_timeout_ms)
  }

  #[cfg(feature = "blocking-encoding")]
  pub fn blocking_encode_threshold(&self) -> usize {
    read_atomic(&self.blocking_encode_threshold)
  }
}

lazy_static! {
  static ref GLOBALS: Globals = Globals::default();
}

pub(crate) fn globals() -> &'static Globals {
  &GLOBALS
}

/// Read the case-sensitive list of error prefixes (without the leading `-`) that will trigger the client to reconnect and retry the last command.
///
/// Default: CLUSTERDOWN, READONLY, LOADING
#[cfg(feature = "custom-reconnect-errors")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom-reconnect-errors")))]
pub fn get_custom_reconnect_errors() -> Vec<ReconnectError> {
  globals().reconnect_errors.read().clone()
}

/// See [get_custom_reconnect_errors] for more information.
#[cfg(feature = "custom-reconnect-errors")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom-reconnect-errors")))]
pub fn set_custom_reconnect_errors(prefixes: Vec<ReconnectError>) {
  let mut guard = globals().reconnect_errors.write();
  *guard = prefixes;
}

/// The minimum size, in bytes, of frames that should be encoded or decoded with a blocking task.
///
/// See [block_in_place](https://docs.rs/tokio/1.9.0/tokio/task/fn.block_in_place.html) for more information.
///
/// Default: 500 Kb
#[cfg(feature = "blocking-encoding")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking-encoding")))]
pub fn get_blocking_encode_threshold() -> usize {
  read_atomic(&globals().blocking_encode_threshold)
}

/// See [get_blocking_encode_threshold] for more information.
#[cfg(feature = "blocking-encoding")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking-encoding")))]
pub fn set_blocking_encode_threshold(val: usize) -> usize {
  set_atomic(&globals().blocking_encode_threshold, val)
}

/// The timeout to apply to connections to sentinel servers.
///
/// Default: 200 ms
pub fn get_sentinel_connection_timeout_ms() -> usize {
  read_atomic(&globals().sentinel_connection_timeout_ms)
}

/// See [get_sentinel_connection_timeout_ms] for more information.
pub fn set_sentinel_connection_timeout_ms(val: usize) -> usize {
  set_atomic(&globals().sentinel_connection_timeout_ms, val)
}
