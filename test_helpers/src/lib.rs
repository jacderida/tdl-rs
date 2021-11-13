pub mod source_port {
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

pub mod date_time {
    use chrono::{DateTime, Utc};
    use std::cell::Cell;

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
}

pub mod cache {
    use chrono::{DateTime, Utc};
    use color_eyre::{Report, Result};
    use std::path::Path;

    pub fn set_date_for_cache_entries<P: AsRef<Path>>(
        cache_directory_path: P,
        date: DateTime<Utc>,
    ) -> Result<(), Report> {
        let entries = std::fs::read_dir(cache_directory_path)?
            .map(|x| x.map(|y| y.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        for path in entries {
            let cache_entry = std::fs::read_to_string(&path)?;
            let cache_entry = cache_entry.replace("__CACHED_DATE__", &date.to_string());
            std::fs::write(&path, cache_entry)?;
        }
        Ok(())
    }
}
