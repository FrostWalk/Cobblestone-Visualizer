use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64};

use lazy_static::lazy_static;

use crate::robots::sharable_runner::SharableRunner;

lazy_static! {
    pub(crate) static ref RUNNER: Arc<RwLock<SharableRunner>>=Arc::new(RwLock::new(SharableRunner::new(None)));
}

pub(crate) static TERMINATED: AtomicBool = AtomicBool::new(false);
pub(crate) static PAUSE: AtomicBool = AtomicBool::new(false);
pub(crate) static WAIT: AtomicU64 = AtomicU64::new(100);