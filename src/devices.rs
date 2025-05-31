// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!RRTK's device system works through a graph-like structure where each device holds objects called
//!terminals in [`RefCell`]s. Terminals represent anywhere that a device can connect to another.
//!Connected terminals hold references to eachother's [`RefCell`]s. This module holds builtin
//!devices.
use crate::*;
pub mod wrappers;
///A device such that positive for one terminal is negative for the other.
///As this device has only one degree of freedom, it propagates [`Command`]s given to its terminals
///as well as [`State`]s.
pub struct Invert<'a, E: Clone + Debug> {
    term1: RefCell<Terminal<'a, E>>,
    term2: RefCell<Terminal<'a, E>>,
}
impl<'a, E: Clone + Debug> Invert<'a, E> {
    ///Constructor for [`Invert`].
    pub const fn new() -> Self {
        Self {
            term1: Terminal::new(),
            term2: Terminal::new(),
        }
    }
    ///Get a reference to the side 1 terminal of the invert device.
    pub const fn get_terminal_1(&self) -> &'a RefCell<Terminal<'a, E>> {
        //We don't want to extend the `&self` reference beyond the scope of the function, but we
        //need need the `term` reference to last for 'a, so we do this to get a reference with a
        //longer lifetime. This should be OK since both terminals are restricted to the 'a
        //lifetime.
        unsafe { &*(&self.term1 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the side 2 terminal of the invert device.
    pub const fn get_terminal_2(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.term2 as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<E: Clone + Debug> Updatable<E> for Invert<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let get1: Option<Datum<State>> = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2: Option<Datum<State>> = self
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
        let get1: Option<Datum<Command>> = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2: Option<Datum<Command>> = self
            .term2
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let mut maybe_datum: Option<Datum<Command>> = None;
        maybe_datum.replace_if_none_or_older_than_option(get1);
        match get2 {
            Some(x) => {
                maybe_datum.replace_if_none_or_older_than(-x);
            }
            None => {}
        }
        match maybe_datum {
            Some(datum_command) => {
                self.term1.borrow_mut().set(datum_command)?;
                self.term2.borrow_mut().set(-datum_command)?;
            }
            None => {}
        }
        Ok(())
    }
}
impl<E: Clone + Debug> Device<E> for Invert<'_, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.term1.borrow_mut().update()?;
        self.term2.borrow_mut().update()?;
        Ok(())
    }
}
///A gear train, a mechanism consisting of a two or more gears meshed together.
///As this device has only one degree of freedom, it propagates [`Command`]s given to its terminals
///as well as [`State`]s.
pub struct GearTrain<'a, E: Clone + Debug> {
    term1: RefCell<Terminal<'a, E>>,
    term2: RefCell<Terminal<'a, E>>,
    ratio: f32,
}
impl<'a, E: Clone + Debug> GearTrain<'a, E> {
    ///Construct a [`GearTrain`] with the ratio as an `f32`.
    pub const fn with_ratio_raw(ratio: f32) -> Self {
        Self {
            term1: Terminal::new(),
            term2: Terminal::new(),
            ratio: ratio,
        }
    }
    ///Construct a [`GearTrain`] with the ratio as a dimensionless [`Quantity`].
    pub const fn with_ratio(ratio: Quantity) -> Self {
        ratio.unit.assert_eq_assume_ok(&DIMENSIONLESS);
        Self::with_ratio_raw(ratio.value)
    }
    ///Construct a [`GearTrain`] from an array of the numbers of teeth on each gear in the train.
    pub const fn new<const N: usize>(teeth: [f32; N]) -> Self {
        if N < 2 {
            panic!(
                "rrtk::devices::GearTrain::new must be provided with at least two gear tooth counts."
            );
        }
        let ratio = teeth[0] / teeth[teeth.len() - 1] * if N % 2 == 0 { -1.0 } else { 1.0 };
        Self::with_ratio_raw(ratio)
    }
    ///Get a reference to the side 1 terminal of the device where (side 1) * ratio = (side 2).
    pub const fn get_terminal_1(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.term1 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the side 2 terminal of the device where (side 1) * ratio = (side 2).
    pub const fn get_terminal_2(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.term2 as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<E: Clone + Debug> Updatable<E> for GearTrain<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let get1: Option<Datum<State>> = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2: Option<Datum<State>> = self
            .term2
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        match get1 {
            Some(datum1) => match get2 {
                Some(datum2) => {
                    let state1 = datum1.value;
                    let state2 = datum2.value;
                    let time = if datum1.time >= datum2.time {
                        datum1.time
                    } else {
                        datum2.time
                    };
                    //https://www.desmos.com/3d/gvwbqszr5e
                    let r_squared_plus_1 = self.ratio * self.ratio + 1.0;
                    let x_plus_r_y = state1 + state2 * self.ratio;
                    let newstate1 = x_plus_r_y / r_squared_plus_1;
                    let newstate2 = (x_plus_r_y * self.ratio) / r_squared_plus_1;
                    self.term1.borrow_mut().set(Datum::new(time, newstate1))?;
                    self.term2.borrow_mut().set(Datum::new(time, newstate2))?;
                }
                None => {
                    let newdatum2 = datum1 * self.ratio;
                    self.term2.borrow_mut().set(newdatum2)?;
                }
            },
            None => match get2 {
                Some(datum2) => {
                    let newdatum1 = datum2 / self.ratio;
                    self.term1.borrow_mut().set(newdatum1)?;
                }
                None => {}
            },
        }
        let get1: Option<Datum<Command>> = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2: Option<Datum<Command>> = self
            .term2
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        match get1 {
            Some(datum1) => match get2 {
                Some(datum2) => {
                    if datum1.time >= datum2.time {
                        let newdatum2 = datum1 * self.ratio;
                        self.term2.borrow_mut().set(newdatum2)?;
                    } else {
                        let newdatum1 = datum2 / self.ratio;
                        self.term1.borrow_mut().set(newdatum1)?;
                    }
                }
                None => {
                    let newdatum2 = datum1 * self.ratio;
                    self.term2.borrow_mut().set(newdatum2)?;
                }
            },
            None => match get2 {
                Some(datum2) => {
                    let newdatum1 = datum2 / self.ratio;
                    self.term1.borrow_mut().set(newdatum1)?;
                }
                None => {}
            },
        }
        Ok(())
    }
}
impl<E: Clone + Debug> Device<E> for GearTrain<'_, E> {
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
///but the intention is that [`connect`] is for only two and [`Axle`] is for more. Using an [`Axle`] for
///only two terminals is possible but may have a slight performance cost. (The type even
///technically allows for only one or even zero connected terminals, but there is almost certainly
///no legitimate use for this.)
///As this device has only one degree of freedom, it propagates [`Command`]s given to its terminals
///as well as [`State`]s.
pub struct Axle<'a, const N: usize, E: Clone + Debug> {
    inputs: [RefCell<Terminal<'a, E>>; N],
}
impl<'a, const N: usize, E: Clone + Debug> Axle<'a, N, E> {
    ///Constructor for [`Axle`].
    pub fn new() -> Self {
        let mut inputs: [core::mem::MaybeUninit<RefCell<Terminal<'a, E>>>; N] =
            [const { core::mem::MaybeUninit::uninit() }; N];
        for i in &mut inputs {
            i.write(Terminal::new());
        }
        //transmute doesn't work well with generics, so this does the same thing through pointers instead.
        let inputs: [RefCell<Terminal<'a, E>>; N] = unsafe {
            inputs
                .as_ptr()
                .cast::<[RefCell<Terminal<'a, E>>; N]>()
                .read()
        };
        Self { inputs: inputs }
    }
    ///Get a reference to one of the axle's terminals.
    pub const fn get_terminal(&self, terminal: usize) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.inputs[terminal] as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<const N: usize, E: Clone + Debug> Updatable<E> for Axle<'_, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let mut count = 0u16;
        let mut datum = Datum::new(Time::from_nanoseconds(i64::MIN), State::default());
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
        let mut maybe_datum: Option<Datum<Command>> = None;
        for i in &self.inputs {
            maybe_datum.replace_if_none_or_older_than_option(i.borrow().get()?);
        }
        if let Some(datum) = maybe_datum {
            for i in &self.inputs {
                i.borrow_mut().set(datum.clone())?;
            }
        }
        Ok(())
    }
}
impl<const N: usize, E: Clone + Debug> Device<E> for Axle<'_, N, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        for i in &self.inputs {
            i.borrow_mut().update()?;
        }
        Ok(())
    }
}
///Since each branch of a differential is dependent on the other two, we can calculate each with
///only the others. This allows you to select a branch to completely calculate and not call
///[`get`](Terminal::get)
///on. For example, if you have encoders on two branches, you would probably want to calculate the
///third from their readings. If you have encoders on all three branches, you can also choose to
///use all three values from them with the [`Equal`](DifferentialDistrust::Equal) variant.
pub enum DifferentialDistrust {
    ///Calculate the state of side 1 from sum and side 2 and do not call [`get`](Terminal::get) on it.
    Side1,
    ///Calculate the state of side 2 from sum and side 1 and do not call [`get`](Terminal::get) on it.
    Side2,
    ///Calculate the state of sum from side 1 and side 2 and do not call [`get`](Terminal::get) on it.
    Sum,
    ///Trust all branches equally in the calculation. Note that this is a bit slower.
    Equal,
}
///A mechanical differential mechanism.
///As this device has two degrees of freedom, it is not able to propagate [`Command`]s given to its
///terminals as it does with [`State`]s.
pub struct Differential<'a, E: Clone + Debug> {
    side1: RefCell<Terminal<'a, E>>,
    side2: RefCell<Terminal<'a, E>>,
    sum: RefCell<Terminal<'a, E>>,
    distrust: DifferentialDistrust,
}
impl<'a, E: Clone + Debug> Differential<'a, E> {
    ///Constructor for [`Differential`]. Trusts all branches equally.
    pub const fn new() -> Self {
        Self {
            side1: Terminal::new(),
            side2: Terminal::new(),
            sum: Terminal::new(),
            distrust: DifferentialDistrust::Equal,
        }
    }
    ///Constructor for [`Differential`] where you choose what to distrust.
    pub const fn with_distrust(distrust: DifferentialDistrust) -> Self {
        Self {
            side1: Terminal::new(),
            side2: Terminal::new(),
            sum: Terminal::new(),
            distrust: distrust,
        }
    }
    ///Get a reference to the side 1 terminal of the differential.
    pub const fn get_side_1(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.side1 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the side 2 terminal of the differential.
    pub const fn get_side_2(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.side2 as *const RefCell<Terminal<'a, E>>) }
    }
    ///Get a reference to the sum terminal of the differential.
    pub const fn get_sum(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.sum as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<E: Clone + Debug> Updatable<E> for Differential<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        match self.distrust {
            DifferentialDistrust::Side1 => {
                let sum: Datum<State> = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side2: Datum<State> = match self.side2.borrow().get()? {
                    Some(side2) => side2,
                    None => return Ok(()),
                };
                self.side1.borrow_mut().set(sum - side2)?;
            }
            DifferentialDistrust::Side2 => {
                let sum: Datum<State> = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side1: Datum<State> = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                self.side2.borrow_mut().set(sum - side1)?;
            }
            DifferentialDistrust::Sum => {
                let side1: Datum<State> = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                let side2: Datum<State> = match self.side2.borrow().get()? {
                    Some(side2) => side2,
                    None => return Ok(()),
                };
                self.sum.borrow_mut().set(side1 + side2)?;
            }
            DifferentialDistrust::Equal => {
                let sum: Datum<State> = match self.sum.borrow().get()? {
                    Some(sum) => sum,
                    None => return Ok(()),
                };
                let side1: Datum<State> = match self.side1.borrow().get()? {
                    Some(side1) => side1,
                    None => return Ok(()),
                };
                let side2: Datum<State> = match self.side2.borrow().get()? {
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
impl<E: Clone + Debug> Device<E> for Differential<'_, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.side1.borrow_mut().update()?;
        self.side2.borrow_mut().update()?;
        self.sum.borrow_mut().update()?;
        Ok(())
    }
}
