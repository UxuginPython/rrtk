// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Streams that convert from one type to another. Some of these also do keep the same type and are
//!for convenience in certain situations, for example when you do not want to handle a `None`
//!variant yourself.
use crate::streams::math::*;
use crate::streams::*;
///A stream converting all `Ok(None)` values from its input to `Err(_)` variants.
pub struct NoneToError<T: Clone, G: Getter<T, E>, E: Copy + Debug> {
    input: Reference<G>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> NoneToError<T, G, E> {
    ///Constructor for `NoneToError`.
    pub fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> Getter<T, E> for NoneToError<T, G, E> {
    fn get(&self) -> Output<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Err(Error::FromNone);
            }
        }
    }
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> Updatable<E> for NoneToError<T, G, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream converting all `Ok(None)` values from its input to a default `Ok(Some(_))` value.
pub struct NoneToValue<T: Clone, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> {
    input: Reference<G>,
    time_getter: Reference<TG>,
    none_value: T,
    phantom_e: PhantomData<E>,
}
impl<T: Clone, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> NoneToValue<T, G, TG, E> {
    ///Constructor for `NoneToValue`.
    pub fn new(input: Reference<G>, time_getter: Reference<TG>, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Clone, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> Getter<T, E>
    for NoneToValue<T, G, TG, E>
{
    fn get(&self) -> Output<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Ok(Some(Datum::new(
                    self.time_getter.borrow().get()?,
                    self.none_value.clone(),
                )))
            }
        }
    }
}
impl<T: Clone, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> Updatable<E>
    for NoneToValue<T, G, TG, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
pub use acceleration_to_state::AccelerationToState;
mod acceleration_to_state {
    use super::*;
    struct Update0 {
        last_update_time: i64,
        acc: f32,
        update_1: Option<Update1>,
    }
    struct Update1 {
        vel: f32,
        update_2: Option<f32>, //position
    }
    ///A stream that integrates an acceleration getter to construct a full state. Mostly useful for
    ///encoders.
    pub struct AccelerationToState<G: Getter<f32, E>, E: Copy + Debug> {
        acc: Reference<G>,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> AccelerationToState<G, E> {
        ///Constructor for `AccelerationToState`.
        pub fn new(acc: Reference<G>) -> Self {
            Self {
                acc: acc,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> Getter<State, E> for AccelerationToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => match update_1.update_2 {
                        Some(position) => Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(position, update_1.vel, update_0.acc),
                        ))),
                        None => Ok(None),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> Updatable<E> for AccelerationToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.acc.borrow().get() {
                Ok(gotten) => match gotten {
                    Some(new_acc_datum) => {
                        let new_time = new_acc_datum.time;
                        let new_acc = new_acc_datum.value;
                        match &self.update {
                            Some(update_0) => {
                                let old_time = update_0.last_update_time;
                                let old_acc = update_0.acc;
                                let delta_time = (new_time - old_time) as f32;
                                let vel_addend = (old_acc + new_acc) / 2.0 * delta_time;
                                match &update_0.update_1 {
                                    Some(update_1) => {
                                        let old_vel = update_1.vel;
                                        let new_vel = old_vel + vel_addend;
                                        let pos_addend = (old_vel + new_vel) / 2.0 * delta_time;
                                        match &update_1.update_2 {
                                            Some(old_pos) => {
                                                self.update = Some(Update0 {
                                                    last_update_time: new_time,
                                                    acc: new_acc,
                                                    update_1: Some(Update1 {
                                                        vel: new_vel,
                                                        update_2: Some(old_pos + pos_addend),
                                                    }),
                                                })
                                            }
                                            None => {
                                                self.update = Some(Update0 {
                                                    last_update_time: new_time,
                                                    acc: new_acc,
                                                    update_1: Some(Update1 {
                                                        vel: new_vel,
                                                        update_2: Some(pos_addend),
                                                    }),
                                                })
                                            }
                                        }
                                    }
                                    None => {
                                        self.update = Some(Update0 {
                                            last_update_time: new_time,
                                            acc: new_acc,
                                            update_1: Some(Update1 {
                                                vel: vel_addend,
                                                update_2: None,
                                            }),
                                        })
                                    }
                                }
                            }
                            None => {
                                self.update = Some(Update0 {
                                    last_update_time: new_time,
                                    acc: new_acc,
                                    update_1: None,
                                });
                            }
                        }
                    }
                    None => (), //This just does nothing if the input gives a None. It does not reset
                                //it or anything.
                },
                Err(error) => {
                    self.update = None;
                    return Err(error);
                }
            }
            Ok(())
        }
    }
}
pub use velocity_to_state::VelocityToState;
mod velocity_to_state {
    use super::*;
    struct Update0 {
        last_update_time: i64,
        vel: f32,
        update_1: Option<Update1>,
    }
    struct Update1 {
        acc: f32,
        pos: f32,
    }
    ///A stream that integrates and derivates a velocity getter to construct a full state. Mostly
    ///useful for encoders.
    pub struct VelocityToState<G: Getter<f32, E>, E: Copy + Debug> {
        vel: Reference<G>,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> VelocityToState<G, E> {
        ///Constructor for `VelocityToState`.
        pub fn new(vel: Reference<G>) -> Self {
            Self {
                vel: vel,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> Getter<State, E> for VelocityToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => Ok(Some(Datum::new(
                        update_0.last_update_time,
                        State::new(update_1.pos, update_0.vel, update_1.acc),
                    ))),
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<f32, E>, E: Copy + Debug> Updatable<E> for VelocityToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            todo!();
        }
    }
}
/*///A stream that derivates a position getter to construct a full state. Mostly useful for encoders.
pub struct PositionToState<E: Copy + Debug> {
    acc: InputGetter<f32, E>,
    vel: InputGetter<f32, E>,
    pos: InputGetter<f32, E>,
}
impl<E: Copy + Debug + 'static> PositionToState<E> {
    ///Constructor for `PositionToState`.
    pub fn new(pos: InputGetter<f32, E>) -> Self {
        let vel = make_input_getter(DerivativeStream::new(Rc::clone(&pos)));
        let acc = make_input_getter(DerivativeStream::new(Rc::clone(&vel)));
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Getter<State, E> for PositionToState<E> {
    fn get(&self) -> Output<State, E> {
        let acc = self.acc.borrow().get()?;
        let vel = self.vel.borrow().get()?;
        let pos = self.pos.borrow().get()?;
        match acc {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match vel {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match pos {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let acc = acc.unwrap();
        let vel = vel.unwrap();
        let pos = pos.unwrap();
        let time = acc.time;
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
}
impl<E: Copy + Debug> Updatable<E> for PositionToState<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.vel.borrow_mut().update()?;
        self.acc.borrow_mut().update()?;
        Ok(())
    }
}*/
