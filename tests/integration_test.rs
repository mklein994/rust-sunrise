// The MIT License (MIT)
//
// Copyright (c) 2018 Nathan Osman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::f64::consts::PI;

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
use chrono::NaiveDate;
#[cfg(all(not(feature = "chrono"), feature = "jiff"))]
use jiff::civil::Date;
use sunrise::{Coordinates, DawnType, SolarDay, SolarEvent};

#[allow(deprecated)]
#[cfg(not(feature = "jiff"))]
use sunrise::sunrise_sunset;

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
fn solar_day(year: i32) -> SolarDay {
    SolarDay::new(
        Coordinates::new(0., 0.).unwrap(),
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
    )
}

#[cfg(all(not(feature = "chrono"), feature = "jiff"))]
fn solar_day(year: i32) -> SolarDay {
    SolarDay::new(
        Coordinates::new(0., 0.).unwrap(),
        Date::new(year.try_into().unwrap(), 1, 1).unwrap(),
    )
}

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
macro_rules! expected_datetime {
    ($value:expr) => {
        chrono::DateTime::parse_from_rfc3339($value).unwrap()
    };
}

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
macro_rules! expected_date {
    ($year:expr, $month:expr, $day:expr) => {
        chrono::NaiveDate::from_ymd_opt($year, $month, $day).unwrap()
    };
}

#[cfg(all(not(feature = "chrono"), feature = "jiff"))]
macro_rules! expected_datetime {
    ($value:expr) => {
        $value.parse().unwrap()
    };
}

#[cfg(all(not(feature = "chrono"), feature = "jiff"))]
macro_rules! expected_date {
    ($year:expr, $month:expr, $day:expr) => {
        jiff::civil::date($year, $month, $day)
    };
}

#[test]
#[allow(deprecated)]
#[cfg(not(feature = "jiff"))]
fn test_sunrise() {
    assert_eq!(sunrise_sunset(0., 0., 1970, 1, 1), (21594, 65228));
    assert_eq!(
        solar_day(1970).event_time(SolarEvent::Sunrise),
        expected_datetime!("1970-01-01T05:59:54Z")
    );

    assert_eq!(
        solar_day(1970).event_time(SolarEvent::Sunset),
        expected_datetime!("1970-01-01T18:07:08Z")
    );
}

#[test]
fn test_altitude() {
    assert_eq!(
        solar_day(1970)
            .with_altitude(123.)
            .event_time(SolarEvent::Sunrise),
        expected_datetime!("1970-01-01T05:58:14Z")
    );

    assert_eq!(
        solar_day(1970)
            .with_altitude(-10.)
            .event_time(SolarEvent::Sunrise),
        expected_datetime!("1970-01-01T06:00:22Z")
    );
}

#[test]
fn test_civil() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Civil)),
        expected_datetime!("2023-01-01T05:37:08Z")
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Civil)),
        expected_datetime!("2023-01-01T18:29:18Z")
    );
}

#[test]
fn test_nautical() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Nautical)),
        expected_datetime!("2023-01-01T05:11:00Z")
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Nautical)),
        expected_datetime!("2023-01-01T18:55:27Z")
    );
}

#[test]
fn test_astronomical() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Astronomical)),
        expected_datetime!("2023-01-01T04:44:45Z")
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Astronomical)),
        expected_datetime!("2023-01-01T19:21:42Z")
    );
}

#[test]
fn test_elevation() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Elevation {
            elevation: PI / 4.0,
            morning: true
        }),
        expected_datetime!("2023-01-01T02:42:24Z")
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Elevation {
            elevation: PI / 4.0,
            morning: false
        }),
        expected_datetime!("2023-01-01T21:24:02Z")
    );
}

#[test]
fn test_order() {
    let sd = {
        SolarDay::new(
            Coordinates::new(2.0, 10.0).unwrap(),
            expected_date!(2024, 2, 23),
        )
        .with_altitude(100.0)
    };

    let events_time = [
        sd.event_time(SolarEvent::Dawn(DawnType::Astronomical)),
        sd.event_time(SolarEvent::Dawn(DawnType::Nautical)),
        sd.event_time(SolarEvent::Dawn(DawnType::Civil)),
        sd.event_time(SolarEvent::Sunrise),
        sd.event_time(SolarEvent::Elevation {
            elevation: -0.1,
            morning: true,
        }),
        sd.event_time(SolarEvent::Elevation {
            elevation: -0.1,
            morning: false,
        }),
        sd.event_time(SolarEvent::Sunset),
        sd.event_time(SolarEvent::Dusk(DawnType::Civil)),
        sd.event_time(SolarEvent::Dusk(DawnType::Nautical)),
        sd.event_time(SolarEvent::Dusk(DawnType::Astronomical)),
    ];

    assert!(events_time.is_sorted());
}
