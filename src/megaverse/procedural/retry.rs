use log::{debug, warn};
use std::time::Duration;
use std::{error, thread};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn exponential_backoff<T>(
    retries: u32,
    initial_time: Duration,
    exponent: u32,
    f: &impl Fn() -> Result<T>,
) -> Result<T> {
    let mut wait_time = initial_time;
    for attempt in 0..=retries {
        match f() {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                if attempt == retries {
                    return Err(
                        "Maximum number of retries reached without a successful response."
                            .to_string()
                            .into(),
                    );
                } else {
                    warn!("Got error {e}, retrying.")
                }
            }
        }

        wait_time *= exponent;
        debug!("Sleeping for {wait_time:?}s.");
        thread::sleep(wait_time);
    }
    unreachable!();
}
