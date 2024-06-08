use robotics_lib::runner::Runner;

pub(crate) struct SharableRunner {
    pub(crate) option_runner: Option<Runner>,
}

unsafe impl Send for SharableRunner {}

unsafe impl Sync for SharableRunner {}

impl SharableRunner {
    pub(crate) fn new(r: Option<Runner>) -> Self {
        Self {
            option_runner: r
        }
    }
}