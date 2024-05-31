use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::robots::sharable_runner::SharableRunner;

lazy_static! {
    pub(crate) static ref RUNNER: Mutex<SharableRunner> = Mutex::new(SharableRunner::new(None));
}

pub(crate) static TERMINATED: AtomicBool = AtomicBool::new(false);
pub(crate) static PAUSE: AtomicBool = AtomicBool::new(false);
pub(crate) static WAIT: AtomicU64 = AtomicU64::new(100);