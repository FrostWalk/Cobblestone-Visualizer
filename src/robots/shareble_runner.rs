use derive_new::new;
use robotics_lib::runner::Runner;

#[derive(new)]
pub(crate) struct SharableRunner {
    pub(crate) runner: Option<Runner>,
}

unsafe impl Sync for SharableRunner {}

unsafe impl Send for SharableRunner {}