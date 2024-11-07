// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!Streams that convert from one type to another. Some of these also do keep the same type and are
//!for convenience in certain situations, for example when you do not want to handle a `None`
//!variant yourself.
use crate::streams::*;
///A stream converting all `Ok(None)` values from its input to `Err(_)` variants.
pub struct NoneToError<T: Clone, G: Getter<T, E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Clone, G: Getter<T, E> + ?Sized, E: Copy + Debug> NoneToError<T, G, E> {
    ///Constructor for `NoneToError`.
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Clone, G: Getter<T, E> + ?Sized, E: Copy + Debug> Getter<T, E> for NoneToError<T, G, E> {
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
impl<T: Clone, G: Getter<T, E> + ?Sized, E: Copy + Debug> Updatable<E> for NoneToError<T, G, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream converting all `Ok(None)` values from its input to a default `Ok(Some(_))` value.
pub struct NoneToValue<
    T: Clone,
    G: Getter<T, E> + ?Sized,
    TG: TimeGetter<E> + ?Sized,
    E: Copy + Debug,
> {
    input: Reference<G>,
    time_getter: Reference<TG>,
    none_value: T,
    phantom_e: PhantomData<E>,
}
impl<T: Clone, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug>
    NoneToValue<T, G, TG, E>
{
    ///Constructor for `NoneToValue`.
    pub const fn new(input: Reference<G>, time_getter: Reference<TG>, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Clone, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug> Getter<T, E>
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
impl<T: Clone, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug> Updatable<E>
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
        last_update_time: Time,
        acc: Quantity,
        update_1: Option<Update1>,
    }
    struct Update1 {
        vel: Quantity,
        update_2: Option<Quantity>, //position
    }
    ///A stream that integrates an acceleration getter to construct a full state. Mostly useful for
    ///encoders.
    pub struct AccelerationToState<G: Getter<f32, E> + ?Sized, E: Copy + Debug> {
        acc: Reference<G>,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> AccelerationToState<G, E> {
        ///Constructor for `AccelerationToState`.
        pub const fn new(acc: Reference<G>) -> Self {
            Self {
                acc: acc,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<State, E> for AccelerationToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => match update_1.update_2 {
                        Some(position) => Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(position.value, update_1.vel.value, update_0.acc.value),
                        ))),
                        None => Ok(None),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E> for AccelerationToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.acc.borrow().get() {
                Ok(gotten) => match gotten {
                    Some(new_acc_datum) => {
                        let new_time = new_acc_datum.time;
                        let new_acc =
                            Quantity::new(new_acc_datum.value, MILLIMETER_PER_SECOND_SQUARED);
                        match &self.update {
                            Some(update_0) => {
                                let old_time = update_0.last_update_time;
                                let old_acc = update_0.acc;
                                let delta_time = Quantity::from(new_time - old_time);
                                let vel_addend =
                                    (old_acc + new_acc) / Quantity::dimensionless(2.0) * delta_time;
                                match &update_0.update_1 {
                                    Some(update_1) => {
                                        let old_vel = update_1.vel;
                                        let new_vel = old_vel + vel_addend;
                                        let pos_addend = (old_vel + new_vel)
                                            / Quantity::dimensionless(2.0)
                                            * delta_time;
                                        match &update_1.update_2 {
                                            Some(old_pos) => {
                                                self.update = Some(Update0 {
                                                    last_update_time: new_time,
                                                    acc: new_acc,
                                                    update_1: Some(Update1 {
                                                        vel: new_vel,
                                                        update_2: Some(*old_pos + pos_addend),
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
        last_update_time: Time,
        vel: Quantity,
        update_1: Option<Update1>,
    }
    struct Update1 {
        acc: Quantity,
        pos: Quantity,
    }
    ///A stream that integrates and derivates a velocity getter to construct a full state. Mostly
    ///useful for encoders.
    pub struct VelocityToState<G: Getter<f32, E> + ?Sized, E: Copy + Debug> {
        vel: Reference<G>,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> VelocityToState<G, E> {
        ///Constructor for `VelocityToState`.
        pub const fn new(vel: Reference<G>) -> Self {
            Self {
                vel: vel,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<State, E> for VelocityToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => Ok(Some(Datum::new(
                        update_0.last_update_time,
                        State::new(
                            update_1.pos.into(),
                            update_0.vel.into(),
                            update_1.acc.into(),
                        ),
                    ))),
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E> for VelocityToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.vel.borrow().get() {
                Ok(gotten) => match gotten {
                    Some(new_vel_datum) => {
                        let new_time = new_vel_datum.time;
                        let new_vel = Quantity::new(new_vel_datum.value, MILLIMETER_PER_SECOND);
                        match &self.update {
                            Some(update_0) => {
                                let old_time = update_0.last_update_time;
                                let delta_time = Quantity::from(new_time - old_time);
                                let old_vel = update_0.vel;
                                let new_acc = (new_vel - old_vel) / delta_time;
                                let pos_addend =
                                    (old_vel + new_vel) / Quantity::dimensionless(2.0) * delta_time;
                                match &update_0.update_1 {
                                    Some(update_1) => {
                                        self.update = Some(Update0 {
                                            last_update_time: new_time,
                                            vel: new_vel,
                                            update_1: Some(Update1 {
                                                acc: new_acc,
                                                pos: update_1.pos + pos_addend,
                                            }),
                                        });
                                    }
                                    None => {
                                        self.update = Some(Update0 {
                                            last_update_time: new_time,
                                            vel: new_vel,
                                            update_1: Some(Update1 {
                                                acc: new_acc,
                                                pos: pos_addend,
                                            }),
                                        });
                                    }
                                }
                            }
                            None => {
                                self.update = Some(Update0 {
                                    last_update_time: new_time,
                                    vel: new_vel,
                                    update_1: None,
                                });
                            }
                        }
                    }
                    None => (),
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
/*pub use position_to_state::PositionToState;
mod position_to_state {
    use super::*;
    struct Update0 {
        last_update_time: i64,
        pos: f32,
        update_1: Option<Update1>,
    }
    struct Update1 {
        vel: f32,
        update_2: Option<f32>, //acceleration
    }
    ///A stream that derivates a position getter to construct a full state. Mostly useful for encoders.
    pub struct PositionToState<G: Getter<f32, E> + ?Sized, E: Copy + Debug> {
        pos: Reference<G>,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> PositionToState<G, E> {
        ///Constructor for `PositionToState`.
        pub const fn new(pos: Reference<G>) -> Self {
            Self {
                pos: pos,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<State, E> for PositionToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => match update_1.update_2 {
                        Some(acc) => Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(update_0.pos, update_1.vel, acc),
                        ))),
                        None => Ok(None),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E> for PositionToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.pos.borrow().get() {
                Ok(gotten) => match gotten {
                    Some(new_pos_datum) => {
                        let new_time = new_pos_datum.time;
                        let new_pos = new_pos_datum.value;
                        match &self.update {
                            Some(update_0) => {
                                let old_time = update_0.last_update_time;
                                let delta_time = (new_time - old_time) as f32;
                                let old_pos = update_0.pos;
                                let new_vel = (new_pos - old_pos) / delta_time;
                                match &update_0.update_1 {
                                    Some(update_1) => {
                                        let old_vel = update_1.vel;
                                        let new_acc = (new_vel - old_vel) / delta_time;
                                        self.update = Some(Update0 {
                                            last_update_time: new_time,
                                            pos: new_pos,
                                            update_1: Some(Update1 {
                                                vel: new_vel,
                                                update_2: Some(new_acc),
                                            }),
                                        });
                                    }
                                    None => {
                                        self.update = Some(Update0 {
                                            last_update_time: new_time,
                                            pos: new_pos,
                                            update_1: Some(Update1 {
                                                vel: new_vel,
                                                update_2: None,
                                            }),
                                        });
                                    }
                                }
                            }
                            None => {
                                self.update = Some(Update0 {
                                    last_update_time: new_time,
                                    pos: new_pos,
                                    update_1: None,
                                });
                            }
                        }
                    }
                    None => (),
                },
                Err(error) => {
                    self.update = None;
                    return Err(error);
                }
            }
            Ok(())
        }
    }
}*/
