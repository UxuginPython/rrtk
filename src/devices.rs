// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!RRTK's device system works through a graph-like structure where each device holds objects called
//!terminals in `RefCell`s. Terminals represent anywhere that a device can connect to another.
//!Connected terminals hold references to eachother's `RefCell`s. This module holds builtin
//!devices.
use crate::*;
pub mod wrappers;
///A device such that positive for one terminal is negative for the other.
pub struct Invert<'a, E: Copy + Debug> {
    term1: RefCell<Terminal<'a, E>>,
    term2: RefCell<Terminal<'a, E>>,
}
impl<'a, E: Copy + Debug> Invert<'a, E> {
    ///Constructor for `Invert`.
    pub fn new() -> Self {
        Self {
            term1: Terminal::new(),
            term2: Terminal::new(),
        }
    }
    ///Get a reference to the side 1 terminal of the invert device.
    pub fn get_terminal_1(&self) -> &'a RefCell<Terminal<'a, E>> {
        //We don't want to extend the `&self` reference beyond the scope of the function, but we
        //need need the `term` reference to last for 'a, so we do this to get a reference with a
        //longer lifetime. This should be OK since both terminals are restricted to the 'a
        //lifetime.
        unsafe { &*(&self.term1 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the side 2 terminal of the invert device.
    pub fn get_terminal_2(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.term2 as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<E: Copy + Debug> Updatable<E> for Invert<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let get1 = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2 = self
            .term2
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        match get1 {
            None => match get2 {
                None => {}
                Some(datum2) => {
                    let newdatum1 = Datum::new(datum2.time, -datum2.value);
                    self.term1.borrow_mut().set(newdatum1)?;
                }
            },
            Some(datum1) => match get2 {
                None => {
                    let newdatum2 = Datum::new(datum1.time, -datum1.value);
                    self.term2.borrow_mut().set(newdatum2)?;
                }
                Some(datum2) => {
                    let state1 = datum1.value;
                    let state2 = datum2.value;
                    let time = if datum1.time >= datum2.time {
                        datum1.time
                    } else {
                        datum2.time
                    };
                    //average with negative state2 as it is inverted from state1
                    let new_state = (state1 - state2) / 2.0;
                    self.term1.borrow_mut().set(Datum::new(time, new_state))?;
                    self.term2.borrow_mut().set(Datum::new(time, -new_state))?;
                }
            },
        }
        Ok(())
    }
}
impl<E: Copy + Debug> Device<E> for Invert<'_, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.term1.borrow_mut().update()?;
        self.term2.borrow_mut().update()?;
        Ok(())
    }
}
///A connection between terminals that are not directly connected, such as when three or more
///terminals are connected. Code-wise, this is almost exactly the same as directly connecting two
///terminals, but this type can connect more than two terminals. There is some freedom in exactly
///what you do with each of these ways of connecting terminals and what they represent physically,
///but the intention is that `connect` is for only two and `Axle` is for more. Using an `Axle` for
///only two terminals is possible but may have a slight performance cost. (The type even
///technically allows for only one or even zero connected terminals, but there is almost certainly
///no legitimate use for this.)
pub struct Axle<'a, const N: usize, E: Copy + Debug> {
    inputs: [RefCell<Terminal<'a, E>>; N],
}
impl<'a, const N: usize, E: Copy + Debug> Axle<'a, N, E> {
    ///Constructor for `Axle`.
    pub fn new() -> Self {
        let mut inputs: [RefCell<Terminal<'a, E>>; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in &mut inputs {
            *i = Terminal::new();
        }
        Self { inputs: inputs }
    }
    ///Get a reference to one of the axle's terminals.
    pub fn get_terminal(&self, terminal: usize) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.inputs[terminal] as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<const N: usize, E: Copy + Debug> Updatable<E> for Axle<'_, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let mut count = 0u16;
        let mut datum = Datum::new(i64::MIN, State::default());
        for i in &self.inputs {
            match i.borrow().get()? {
                Some(gotten_datum) => {
                    datum += gotten_datum;
                    count += 1;
                }
                None => (),
            }
        }
        if count >= 1 {
            datum /= count as f32;
            for i in &self.inputs {
                i.borrow_mut().set(datum.clone())?;
            }
        }
        Ok(())
    }
}
impl<const N: usize, E: Copy + Debug> Device<E> for Axle<'_, N, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        for i in &self.inputs {
            i.borrow_mut().update()?;
        }
        Ok(())
    }
}
///Since each branch of a differential is dependent on the other two, we can calculate each with
///only the others. This allows you to select a branch to completely calculate and not call `get`
///on. For example, if you have encoders on two branches, you would probably want to calculate the
///third from their readings. If you have encoders on all three branches, you can also choose to
///use all three values from them with the `Equal` variant.
pub enum DifferentialDistrust {
    ///Calculate the state of side 1 from sum and side 2 and do not call `get` on it.
    Side1,
    ///Calculate the state of side 2 from sum and side 1 and do not call `get` on it.
    Side2,
    ///Calculate the state of sum from side 1 and side 2 and do not call `get` on it.
    Sum,
    ///Trust all branches equally in the calculation. Note that this is a bit slower.
    Equal,
}
///A mechanical differential mechanism.
pub struct Differential<'a, E: Copy + Debug> {
    side1: RefCell<Terminal<'a, E>>,
    side2: RefCell<Terminal<'a, E>>,
    sum: RefCell<Terminal<'a, E>>,
    distrust: DifferentialDistrust,
}
impl<'a, E: Copy + Debug> Differential<'a, E> {
    ///Constructor for `Differential`. Trusts all branches equally.
    pub fn new() -> Self {
        Self {
            side1: Terminal::new(),
            side2: Terminal::new(),
            sum: Terminal::new(),
            distrust: DifferentialDistrust::Equal,
        }
    }
    ///Constructor for `Differential` where you choose what to distrust.
    pub fn with_distrust(distrust: DifferentialDistrust) -> Self {
        Self {
            side1: Terminal::new(),
            side2: Terminal::new(),
            sum: Terminal::new(),
            distrust: distrust,
        }
    }
    ///Get a reference to the side 1 terminal of the differential.
    pub fn get_side_1(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.side1 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the side 2 terminal of the differential.
    pub fn get_side_2(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.side2 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the sum terminal of the differential.
    pub fn get_sum(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.sum as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<E: Copy + Debug> Updatable<E> for Differential<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        match self.distrust {
            DifferentialDistrust::Side1 => {
                let sum = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side2 = match self.side2.borrow().get()? {
                    Some(side2) => side2,
                    None => return Ok(()),
                };
                self.side1.borrow_mut().set(sum - side2)?;
            }
            DifferentialDistrust::Side2 => {
                let sum = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side1 = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                self.side2.borrow_mut().set(sum - side1)?;
            }
            DifferentialDistrust::Sum => {
                let side1 = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                let side2 = match self.side2.borrow().get()? {
                    Some(side2) => side2,
                    None => return Ok(()),
                };
                self.sum.borrow_mut().set(side1 + side2)?;
            }
            DifferentialDistrust::Equal => {
                let sum = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side1 = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                let side2 = match self.side2.borrow().get()? {
                    Some(side2) => side2,
                    None => return Ok(()),
                };
                //This minimizes (x-a)^2+(y-b)^2+(z-c)^2 given a+b=c where x, y, and z are the
                //measured values of side1, side2, and sum respectively and a, b, and c are their
                //calculated estimated values based on all three constrained to add. This
                //essentially means that the estimated values will be as close to the measured
                //values as possible while forcing the two sides to add to the sum branch.
                self.sum
                    .borrow_mut()
                    .set((side1 + side2 + sum * 2.0) / 3.0)?;
                self.side1
                    .borrow_mut()
                    .set((side1 * 2.0 - side2 + sum) / 3.0)?;
                self.side2
                    .borrow_mut()
                    .set((-side1 + side2 * 2.0 + sum) / 3.0)?;
            }
        }
        Ok(())
    }
}
impl<E: Copy + Debug> Device<E> for Differential<'_, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.side1.borrow_mut().update()?;
        self.side2.borrow_mut().update()?;
        self.sum.borrow_mut().update()?;
        Ok(())
    }
}
