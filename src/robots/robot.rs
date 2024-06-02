use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::time::Duration;

use common_messages::robot::RobotData;
use log::{info, warn};
use robotics_lib::runner::Runner;

use crate::robots::sharable_runner::SharableRunner;
use crate::robots::variables::{PAUSE, RUNNER, TERMINATED, WAIT};

const LOCK_ERROR: &str = "Unable to lock RUNNER";

pub(crate) fn set_robot(runner: Runner) {
    *RUNNER.lock().expect("unable to gain write access to RUNNER") = SharableRunner::new(Some(runner));
}

pub(crate) fn run_robot() {
    TERMINATED.store(false, SeqCst);
    
    tokio::spawn(async move {
        let wait = get_wait();
        info!("Starting the robot");
        loop {
            if PAUSE.load(Relaxed) {
                if TERMINATED.load(Relaxed) {
                    info!("Stopping the robot, Bye Bye");
                    return;
                }

                tokio::time::sleep(wait / 2).await;

                continue;
            }

            match RUNNER.lock().expect(LOCK_ERROR).option_runner.as_mut() {
                None => {
                    warn!("Start was called but no robot was found");
                    return;
                }
                Some(r) => {
                    r.game_tick().expect("Game tick failed");
                }
            }

            if TERMINATED.load(Relaxed) {
                info!("Stopping the robot, Bye Bye");
                return;
            }

            tokio::time::sleep(wait).await;
        }
    });
}

pub(crate) fn stop_robot() {
    TERMINATED.store(true, SeqCst)
}

pub(crate) fn pause_robot() {
    PAUSE.store(true, SeqCst)
}

pub(crate) fn resume_robot() {
    PAUSE.store(false, SeqCst)
}

pub(crate) fn get_wait() -> Duration {
    Duration::from_millis(WAIT.load(Relaxed))
}

pub(crate) fn get_robot_data() -> RobotData {
    RobotData::from(RUNNER.lock().expect(LOCK_ERROR).option_runner.as_ref().expect("trying to read data from a None robot").get_robot())
}