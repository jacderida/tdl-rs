use chrono::{DateTime, Utc};
use std::cell::Cell;

pub mod helpers {
    #[cfg(target_family = "unix")]
    const FAKE_SOURCE_PORT_BIN_NAME: &str = "fake_source_port";
    #[cfg(target_family = "windows")]
    const FAKE_SOURCE_PORT_BIN_NAME: &str = "fake_source_port.exe";

    pub fn get_fake_source_port_path() -> String {
        let mut fake_source_port_path = std::env::current_dir().unwrap();
        fake_source_port_path.push("target");
        fake_source_port_path.push("debug");
        fake_source_port_path.push(FAKE_SOURCE_PORT_BIN_NAME);
        String::from(fake_source_port_path.as_path().to_str().unwrap())
    }
}

thread_local! {
    static CURRENT_DATE_TIME: Cell<DateTime<Utc>> = Cell::new(Utc::now());
}

pub struct FakeUtc {}
impl FakeUtc {
    pub fn set_date_time(to: DateTime<Utc>) -> Result<(), String> {
        CURRENT_DATE_TIME.with(|d| {
            d.set(to);
        });
        Ok(())
    }

    pub fn now() -> DateTime<Utc> {
        CURRENT_DATE_TIME.with(|d| {
            return d.get();
        })
    }
}
