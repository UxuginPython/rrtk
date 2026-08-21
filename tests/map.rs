// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use rrtk::prelude::*;
use rrtk::{DimensionlessInteger, Time};
#[test]
fn datum() {
    assert_eq!(
        Datum::new(Time::from_nanoseconds(2_000_000_000), 5).map(|x| x * 2),
        Datum::new(Time::from_nanoseconds(2_000_000_000), 10)
    );
}
#[test]
fn option_datum() {
    assert_eq!(
        Some(Datum::new(Time::from_nanoseconds(2_000_000_000), 5)).map_value(|x| x * 2),
        Some(Datum::new(Time::from_nanoseconds(2_000_000_000), 10))
    );
    assert_eq!(None::<Datum<u8>>.map_value(|x| x * 2), None,);
}
#[test]
fn output_ok() {
    assert_eq!(
        Ok::<_, ()>(None::<Datum<u8>>).map_ok(|option| if option.is_none() {
            Some(Datum::new(Time::from_nanoseconds(500_000_000_000), 20.9))
        } else {
            None
        }),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(500_000_000_000),
            20.9
        )))
    );
    assert_eq!(
        Ok::<_, ()>(Some(Datum::new(Time::from_nanoseconds(50), 89))).map_ok(|option| {
            if option.is_none() {
                Some(Datum::new(Time::from_nanoseconds(500_000_000_000), 20.9))
            } else {
                None
            }
        }),
        Ok(None)
    );
    assert_eq!(
        Err::<Option<Datum<()>>, u16>(73).map_ok(|option| {
            if option.is_none() {
                Some(Datum::new(Time::from_nanoseconds(500_000_000_000), 20.9))
            } else {
                None
            }
        }),
        Err(73)
    );
}
#[test]
fn output_ok_some() {
    assert_eq!(
        Err::<Option<Datum<u8>>, _>(67)
            .map_ok_some(|datum| Datum::new(datum.time * DimensionlessInteger(2), datum.value)),
        Err(67)
    );
    assert_eq!(
        Ok::<Option<Datum<u8>>, i16>(None)
            .map_ok_some(|datum| Datum::new(datum.time * DimensionlessInteger(2), datum.value)),
        Ok(None)
    );
    assert_eq!(
        Ok::<Option<Datum<u8>>, i16>(Some(Datum::new(
            Time::from_nanoseconds(100_000_000_000),
            20
        )))
        .map_ok_some(|datum| Datum::new(datum.time * DimensionlessInteger(2), datum.value)),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(200_000_000_000),
            20
        )))
    );
}
#[test]
fn output_ok_some_value() {
    assert_eq!(
        Err::<Option<Datum<u8>>, _>(67).map_ok_some_value(|value| value + 5),
        Err(67)
    );
    assert_eq!(
        Ok::<Option<Datum<u8>>, i16>(None).map_ok_some_value(|value| value + 5),
        Ok(None)
    );
    assert_eq!(
        Ok::<Option<Datum<u8>>, i16>(Some(Datum::new(
            Time::from_nanoseconds(100_000_000_000),
            20
        )))
        .map_ok_some_value(|value| value + 5),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(100_000_000_000),
            25
        )))
    );
}
