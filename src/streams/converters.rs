// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Streams that convert from one type to another. Some of these also do keep the same type and are
//!for convenience in certain situations, for example when you do not want to handle a [`None`]
//!variant yourself.
use crate::compile_time_integer::Integer;
use crate::streams::*;
///A stream converting all `Ok(None)` values from its input to `Err(_)` variants.
pub struct NoneToError<T, G, E>
where
    T: Clone,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    input: G,
    from_none: E,
    phantom_t: PhantomData<T>,
}
impl<T, G, E> NoneToError<T, G, E>
where
    T: Clone,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    ///Constructor for [`NoneToError`].
    pub const fn new(input: G, from_none: E) -> Self {
        Self {
            input: input,
            from_none: from_none,
            phantom_t: PhantomData,
        }
    }
}
impl<T, G, E> Getter<T, E> for NoneToError<T, G, E>
where
    T: Clone,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let output = self.input.get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Err(self.from_none.clone());
            }
        }
    }
}
impl<T, G, E> Updatable<E> for NoneToError<T, G, E>
where
    T: Clone,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream converting all `Ok(None)` values from its input to a default `Ok(Some(_))` value.
pub struct NoneToValue<T, G, TG, E>
where
    T: Clone,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    input: G,
    time_getter: TG,
    none_value: T,
    phantom_e: PhantomData<E>,
}
impl<T, G, TG, E> NoneToValue<T, G, TG, E>
where
    T: Clone,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    ///Constructor for [`NoneToValue`].
    pub const fn new(input: G, time_getter: TG, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G, TG, E> Getter<T, E> for NoneToValue<T, G, TG, E>
where
    T: Clone,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let output = self.input.get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Ok(Some(Datum::new(
                    self.time_getter.get()?,
                    self.none_value.clone(),
                )));
            }
        }
    }
}
impl<T, G, TG, E> Updatable<E> for NoneToValue<T, G, TG, E>
where
    T: Clone,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Converts all `Ok(None)` values to `Ok(Some(T::default()))`.
pub struct NoneToDefault<T, G, TG, E>
where
    T: Default,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    input: G,
    time_getter: TG,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, G, TG, E> NoneToDefault<T, G, TG, E>
where
    T: Default,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    ///Constructor for `NoneToDefault`.
    pub const fn new(input: G, time_getter: TG) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G, TG, E> Getter<T, E> for NoneToDefault<T, G, TG, E>
where
    T: Default,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        Ok(Some(match self.input.get()? {
            Some(value) => value,
            None => Datum::new(self.time_getter.get()?, T::default()),
        }))
    }
}
impl<T, G, TG, E> Updatable<E> for NoneToDefault<T, G, TG, E>
where
    T: Default,
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
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
    pub struct AccelerationToState<G: Getter<Quantity, E>, E: Clone + Debug> {
        acc: G,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> AccelerationToState<G, E> {
        ///Constructor for [`AccelerationToState`].
        pub const fn new(acc: G) -> Self {
            Self {
                acc: acc,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Getter<State, E> for AccelerationToState<G, E> {
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
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Updatable<E> for AccelerationToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.acc.get() {
                Ok(gotten) => match gotten {
                    Some(new_acc_datum) => {
                        let new_time = new_acc_datum.time;
                        let new_acc = new_acc_datum.value;
                        new_acc
                            .unit
                            .assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);
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
    pub struct VelocityToState<G: Getter<Quantity, E>, E: Clone + Debug> {
        vel: G,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> VelocityToState<G, E> {
        ///Constructor for [`VelocityToState`].
        pub const fn new(vel: G) -> Self {
            Self {
                vel: vel,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Getter<State, E> for VelocityToState<G, E> {
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
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Updatable<E> for VelocityToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.vel.get() {
                Ok(gotten) => match gotten {
                    Some(new_vel_datum) => {
                        let new_time = new_vel_datum.time;
                        let new_vel = new_vel_datum.value;
                        new_vel.unit.assert_eq_assume_ok(&MILLIMETER_PER_SECOND);
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
pub use position_to_state::PositionToState;
mod position_to_state {
    use super::*;
    struct Update0 {
        last_update_time: Time,
        pos: Quantity,
        update_1: Option<Update1>,
    }
    struct Update1 {
        vel: Quantity,
        update_2: Option<Quantity>, //acceleration
    }
    ///A stream that derivates a position getter to construct a full state. Mostly useful for encoders.
    pub struct PositionToState<G: Getter<Quantity, E>, E: Clone + Debug> {
        pos: G,
        update: Option<Update0>,
        phantom_e: PhantomData<E>,
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> PositionToState<G, E> {
        ///Constructor for [`PositionToState`].
        pub const fn new(pos: G) -> Self {
            Self {
                pos: pos,
                update: None,
                phantom_e: PhantomData,
            }
        }
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Getter<State, E> for PositionToState<G, E> {
        fn get(&self) -> Output<State, E> {
            match &self.update {
                Some(update_0) => match &update_0.update_1 {
                    Some(update_1) => match update_1.update_2 {
                        Some(acc) => Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(update_0.pos.into(), update_1.vel.into(), acc.into()),
                        ))),
                        None => Ok(None),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            }
        }
    }
    impl<G: Getter<Quantity, E>, E: Clone + Debug> Updatable<E> for PositionToState<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            match self.pos.get() {
                Ok(gotten) => match gotten {
                    Some(new_pos_datum) => {
                        let new_time = new_pos_datum.time;
                        let new_pos = new_pos_datum.value;
                        new_pos.unit.assert_eq_assume_ok(&MILLIMETER);
                        match &self.update {
                            Some(update_0) => {
                                let old_time = update_0.last_update_time;
                                let delta_time = Quantity::from(new_time - old_time);
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
}
///Stream to convert an [`f32`] to a [`Quantity`] with a given [`Unit`].
pub struct FloatToQuantity<G: Getter<f32, E>, E: Clone + Debug> {
    unit: Unit,
    input: G,
    phantom_e: PhantomData<E>,
}
impl<G: Getter<f32, E>, E: Clone + Debug> FloatToQuantity<G, E> {
    ///Constructor for [`FloatToQuantity`].
    pub fn new(unit: Unit, input: G) -> Self {
        Self {
            unit: unit,
            input: input,
            phantom_e: PhantomData,
        }
    }
}
impl<G: Getter<f32, E>, E: Clone + Debug> Updatable<E> for FloatToQuantity<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
impl<G: Getter<f32, E>, E: Clone + Debug> Getter<Quantity, E> for FloatToQuantity<G, E> {
    fn get(&self) -> Output<Quantity, E> {
        match self.input.get()? {
            None => Ok(None),
            Some(x) => Ok(Some(Datum::new(x.time, Quantity::new(x.value, self.unit)))),
        }
    }
}
///Stream to convert a [`Quantity`] to a raw [`f32`].
pub struct QuantityToFloat<G: Getter<Quantity, E>, E: Clone + Debug> {
    input: G,
    phantom_e: PhantomData<E>,
}
impl<G: Getter<Quantity, E>, E: Clone + Debug> QuantityToFloat<G, E> {
    ///Constructor for [`QuantityToFloat`].
    pub fn new(input: G) -> Self {
        Self {
            input: input,
            phantom_e: PhantomData,
        }
    }
}
impl<G: Getter<Quantity, E>, E: Clone + Debug> Getter<f32, E> for QuantityToFloat<G, E> {
    fn get(&self) -> Output<f32, E> {
        match self.input.get()? {
            None => Ok(None),
            Some(x) => Ok(Some(Datum::new(x.time, x.value.value))),
        }
    }
}
impl<G: Getter<Quantity, E>, E: Clone + Debug> Updatable<E> for QuantityToFloat<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
//TODO: Decide if you want to make this and DimensionRemover use where clauses too. It makes it a
//bit less clear what's a real type vs what's just a compile-time integer, but it's more in line
//with the other types and might be a bit easier to read.
///Adds a compile-time [`Quantity`](compile_time_dimensions::Quantity) wrapper with a specific unit
///around a number.
pub struct DimensionAdder<T, MM: Integer, S: Integer, G: Getter<T, E>, E: Clone + Debug> {
    input: G,
    phantom_t: PhantomData<T>,
    phantom_mm: PhantomData<MM>,
    phantom_s: PhantomData<S>,
    phantom_e: PhantomData<E>,
}
impl<T, MM: Integer, S: Integer, G: Getter<T, E>, E: Clone + Debug> DimensionAdder<T, MM, S, G, E> {
    ///Constructor for `DimensionAdder`.
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            phantom_t: PhantomData,
            phantom_mm: PhantomData,
            phantom_s: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, MM: Integer, S: Integer, G: Getter<T, E>, E: Clone + Debug>
    Getter<compile_time_dimensions::Quantity<T, MM, S>, E> for DimensionAdder<T, MM, S, G, E>
{
    fn get(&self) -> Output<compile_time_dimensions::Quantity<T, MM, S>, E> {
        match self.input.get()? {
            None => Ok(None),
            Some(x) => Ok(Some(Datum::new(
                x.time,
                compile_time_dimensions::Quantity::new(x.value),
            ))),
        }
    }
}
impl<T, MM: Integer, S: Integer, G: Getter<T, E>, E: Clone + Debug> Updatable<E>
    for DimensionAdder<T, MM, S, G, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Gets the inner number from the output of a getter returning compile-time
///[`Quantity`](compile_time_dimensions::Quantity).
pub struct DimensionRemover<
    T,
    MM: Integer,
    S: Integer,
    G: Getter<compile_time_dimensions::Quantity<T, MM, S>, E>,
    E: Clone + Debug,
> {
    input: G,
    phantom_t: PhantomData<T>,
    phantom_mm: PhantomData<MM>,
    phantom_s: PhantomData<S>,
    phantom_e: PhantomData<E>,
}
impl<
    T,
    MM: Integer,
    S: Integer,
    G: Getter<compile_time_dimensions::Quantity<T, MM, S>, E>,
    E: Clone + Debug,
> DimensionRemover<T, MM, S, G, E>
{
    ///Constructor for `DimensionRemover`.
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            phantom_t: PhantomData,
            phantom_mm: PhantomData,
            phantom_s: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<
    T,
    MM: Integer,
    S: Integer,
    G: Getter<compile_time_dimensions::Quantity<T, MM, S>, E>,
    E: Clone + Debug,
> Getter<T, E> for DimensionRemover<T, MM, S, G, E>
{
    fn get(&self) -> Output<T, E> {
        match self.input.get()? {
            None => Ok(None),
            Some(x) => Ok(Some(Datum::new(x.time, x.value.into_inner()))),
        }
    }
}
impl<
    T,
    MM: Integer,
    S: Integer,
    G: Getter<compile_time_dimensions::Quantity<T, MM, S>, E>,
    E: Clone + Debug,
> Updatable<E> for DimensionRemover<T, MM, S, G, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Converts the output of a getter to another type through [`Into`]. Leaves the timestamp the same
///and passes through `Err(_)` and `Ok(None)` identically.
pub struct IntoConverter<TI, G: Getter<TI, E>, E: Clone + Debug> {
    input: G,
    phantom_ti: PhantomData<TI>,
    phantom_e: PhantomData<E>,
}
impl<TI, G: Getter<TI, E>, E: Clone + Debug> IntoConverter<TI, G, E> {
    ///Constructor for `IntoConverter`.
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            phantom_ti: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<TI, TO, G, E> Getter<TO, E> for IntoConverter<TI, G, E>
where
    TI: Into<TO>,
    G: Getter<TI, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<TO, E> {
        Ok(self
            .input //Result<Option<Datum<T>>, E>
            .get()? //Option<Datum<T>>
            .map(|datum| /*Datum<T>*/ Datum::new(datum.time, datum.value.into())))
    }
}
impl<TI, G: Getter<TI, E>, E: Clone + Debug> Updatable<E> for IntoConverter<TI, G, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Converts errors returned by a getter to another type through [`Into`]. Leaves `Ok` values
///unchanged.
pub struct ErrorIntoConverter<T, G: Getter<T, EI>, EI: Copy + Debug> {
    input: G,
    phantom_t: PhantomData<T>,
    phantom_ei: PhantomData<EI>,
}
impl<T, G: Getter<T, EI>, EI: Copy + Debug> ErrorIntoConverter<T, G, EI> {
    ///Constructor for `ErrorIntoConverter`.
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            phantom_t: PhantomData,
            phantom_ei: PhantomData,
        }
    }
}
impl<T, G, EI, EO> Getter<T, EO> for ErrorIntoConverter<T, G, EI>
where
    G: Getter<T, EI>,
    EI: Copy + Debug + Into<EO>,
    EO: Copy + Debug,
{
    fn get(&self) -> Output<T, EO> {
        self.input.get().map_err(|error| error.into())
    }
}
impl<T, G: Getter<T, EI>, EI: Copy + Debug + Into<EO>, EO: Copy + Debug> Updatable<EO>
    for ErrorIntoConverter<T, G, EI>
{
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<EO> {
        Ok(())
    }
}
