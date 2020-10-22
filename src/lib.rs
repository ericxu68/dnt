extern crate chrono;

use chrono::{FixedOffset, Local, Utc};

pub enum DnT<T> {
    Local,
    ZoneEast(T),
    ZoneWest(T),
}
impl DnT<f64> {
    pub fn date(time_difference: DnT<f64>) -> String {
        match time_difference {
            DnT::Local => Local::now().format("%d-%m-%Y").to_string(),
            DnT::ZoneEast(time_difference) => Utc::now()
                .date()
                .with_timezone(&FixedOffset::east((time_difference * 1800.0 * 2.0) as i32))
                .format("%d-%m-%Y")
                .to_string(),
            DnT::ZoneWest(time_difference) => Utc::now()
                .date()
                .with_timezone(&FixedOffset::west((time_difference * 1800.0 * 2.0) as i32))
                .format("%d-%m-%Y")
                .to_string(),
        }
    }

    pub fn time(time_difference: DnT<f64>) -> String {
        match time_difference {
            DnT::Local => Local::now().format("%H:%M:%S").to_string(),
            DnT::ZoneEast(time_difference) => Utc::now()
                .with_timezone(&FixedOffset::east((time_difference * 1800.0 * 2.0) as i32))
                .time()
                .format("%H:%M:%S")
                .to_string(),
            DnT::ZoneWest(time_difference) => Utc::now()
                .with_timezone(&FixedOffset::west((time_difference * 1800.0 * 2.0) as i32))
                .time()
                .format("%H:%M:%S")
                .to_string(),
        }
    }

    pub fn dateandtime(time_difference: DnT<f64>) -> String {
        match time_difference {
            DnT::Local => Local::now().format("%d-%m-%Y %H:%M:%S").to_string(),
            DnT::ZoneEast(time_difference) => Utc::now()
                .with_timezone(&FixedOffset::east((time_difference * 1800.0 * 2.0) as i32))
                .format("%d-%m-%Y %H:%M:%S")
                .to_string(),
            DnT::ZoneWest(time_difference) => Utc::now()
                .with_timezone(&FixedOffset::west((time_difference * 1800.0 * 2.0) as i32))
                .format("%d-%m-%Y %H:%M:%S")
                .to_string(),
        }
    }
}