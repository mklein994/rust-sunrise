pub trait SunriseDateish<D, T> {
    fn into_timestamp(date: D, time: T) -> i64;

    fn from_timestamp(secs: i64) -> crate::SunriseTimestamp;
}

pub struct DateUtil;

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
impl SunriseDateish<chrono::NaiveDate, chrono::NaiveTime> for DateUtil {
    fn into_timestamp(date: chrono::NaiveDate, time: chrono::NaiveTime) -> i64 {
        date.and_time(time).and_utc().timestamp()
    }

    fn from_timestamp(secs: i64) -> crate::SunriseTimestamp {
        chrono::DateTime::from_timestamp(secs, 0).expect("invalid result")
    }
}

#[cfg(all(not(feature = "chrono"), feature = "jiff"))]
impl SunriseDateish<jiff::civil::Date, jiff::civil::Time> for DateUtil {
    fn into_timestamp(date: jiff::civil::Date, time: jiff::civil::Time) -> i64 {
        date.to_datetime(time)
            .to_zoned(jiff::tz::TimeZone::UTC)
            .unwrap()
            .timestamp()
            .as_second()
    }

    fn from_timestamp(secs: i64) -> crate::SunriseTimestamp {
        jiff::Timestamp::new(secs, 0).expect("invalid result")
    }
}
