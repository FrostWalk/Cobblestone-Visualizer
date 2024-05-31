use std::sync::atomic::Ordering::Relaxed;

use robotics_lib::runner::Runner;

use crate::robots::shareble_runner::SharableRunner;
use crate::robots::variables::{PAUSE, RUNNER, TERMINATED, WAIT};

pub(crate) fn set_robot(runner: Runner) {
    *RUNNER.lock().expect("unable to gain write access to RUNNER") = SharableRunner::new(Some(runner));
}

pub(crate) fn run_robot() {
    tokio::spawn(async move {
        loop {
            if PAUSE.load(Relaxed) {
                continue;
            }

            match RUNNER.lock().expect("Unable to lock RUNNER").runner.as_mut() {
                None => {}
                Some(r) => {
                    r.game_tick().expect("Game tick failed");
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(WAIT.load(Relaxed))).await;

            if TERMINATED.load(Relaxed) {
                break;
            }
        }
    });
}