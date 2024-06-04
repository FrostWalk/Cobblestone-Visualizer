use derive_new::new;
use robotics_lib::runner::Runner;

#[derive(new)]
pub(crate) struct SharableRunner {
    pub(crate) option_runner: Option<Runner>,
}

unsafe impl Send for SharableRunner {}

unsafe impl Sync for SharableRunner {}