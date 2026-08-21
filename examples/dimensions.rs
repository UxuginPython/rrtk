// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use rrtk::*;
fn main() {
    //Let's say your robot goes 300 mm/s and it needs to go 430 mm.
    let distance = Millimeter::new(430.0);
    let speed = MillimeterPerSecond::new(300.0);
    //It takes this long:
    let time: Second<_> = distance / speed;
    println!("{} / {} = {}", distance, speed, time);
    //But what if instead you run your robot at 300 mm/s for a constant time of 2 seconds?
    let new_time = Time::from_nanoseconds(2_000_000_000);
    //It goes this far:
    let new_distance: Millimeter<_> = speed * new_time;
    println!("{} * {} = {}", speed, new_time.as_seconds(), new_distance);
}
