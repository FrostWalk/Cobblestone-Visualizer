use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::time::Duration;

use common_messages::events::LibEvent;
use log::{error, info, warn};
use queues::IsQueue;
use robotics_lib::event::events::Event;
use robotics_lib::runner::Runner;

use crate::robots::sharable_runner::SharableRunner;
use crate::robots::variables::{PAUSE, RUNNER, TERMINATED, WAIT};

pub(crate) fn set_robot(runner: Runner) {
    *RUNNER.lock().expect("unable to gain write access to RUNNER") = SharableRunner::new(Some(runner));
}

pub(crate) fn run_robot() {
    tokio::spawn(async move {
        let wait = get_wait();
        loop {
            if PAUSE.load(Relaxed) {
                if TERMINATED.load(Relaxed) {
                    info!("Stopping the robot, Bye Bye");
                    return;
                }

                tokio::time::sleep(wait).await;

                continue;
            }

            match RUNNER.lock().expect("Unable to lock RUNNER").runner.as_mut() {
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

            info!("Robot is running");
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

/*fn add_event_to_queue(event: Event) {
    EVENTS_QUEUE.write().expect("Enable to lock queue in write mode (add_event_to_queue)")
        .add(LibEvent::from(event)).expect("Unable to add event to queue");
}

pub fn get_event_from_queue() -> Option<LibEvent> {
    if get_queue_size() == 0 {
        return None;
    }

    return match EVENTS_QUEUE.write().expect("Unable to lock queue in write mode (get_event)").remove() {
        Ok(event) => { Some(event) }
        Err(error) => {
            error!("Queue error: {}", error);
            None
        }
    };
}

pub fn get_queue_size() -> usize {
    EVENTS_QUEUE.read().expect("Unable to lock queue in read mode (get_event)").size()
}*/