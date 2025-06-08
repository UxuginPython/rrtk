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
            input,
            from_none,
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
            Some(_) => Ok(output),
            None => Err(self.from_none.clone()),
        }
    }
}
impl<T, G, E> Updatable<E> for NoneToError<T, G, E>
where
    T: Clone,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
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
            input,
            time_getter,
            none_value,
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
            Some(_) => Ok(output),
            None => Ok(Some(Datum::new(
                self.time_getter.get()?,
                self.none_value.clone(),
            ))),
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
        self.time_getter.update()?;
        self.input.update()?;
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
            input,
            time_getter,
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
        self.time_getter.update()?;
        self.input.update()?;
        Ok(())
    }
}
pub use acceleration_to_state::*;
mod acceleration_to_state {
    use super::*;
    struct Update0 {
        last_update_time: Time,
        acceleration: MillimeterPerSecondSquared<f32>,
        update_1: Option<Update1>,
    }
    struct Update1 {
        velocity: MillimeterPerSecond<f32>,
        update_2_position: Option<Millimeter<f32>>,
    }
    ///Doubly integrates an acceleration to create a full [`State`] object. Uses trapezoidal
    ///integration.
    pub struct AccelerationToState<G> {
        input: G,
        update_0: Option<Update0>,
    }
    impl<G> AccelerationToState<G> {
        ///Constructor for `AccelerationToState`.
        pub const fn new(input: G) -> Self {
            Self {
                input,
                update_0: None,
            }
        }
    }
    impl<G, E: Clone + Debug> Getter<State, E> for AccelerationToState<G>
    where
        Self: Updatable<E>,
    {
        fn get(&self) -> Output<State, E> {
            if let Some(update_0) = &self.update_0 {
                if let Some(update_1) = &update_0.update_1 {
                    if let Some(update_2_position) = update_1.update_2_position {
                        return Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(update_2_position, update_1.velocity, update_0.acceleration),
                        )));
                    }
                }
            }
            Ok(None)
        }
    }
    impl<G: Getter<MillimeterPerSecondSquared<f32>, E>, E: Clone + Debug> Updatable<E>
        for AccelerationToState<G>
    {
        fn update(&mut self) -> NothingOrError<E> {
            self.input.update()?;
            match self.input.get() {
                Ok(Some(new_acceleration_datum)) => {
                    let new_update_time = new_acceleration_datum.time;
                    let new_acceleration = new_acceleration_datum.value;
                    self.update_0 = Some(Update0 {
                        last_update_time: new_update_time,
                        acceleration: new_acceleration,
                        update_1: if let Some(update_0) = &self.update_0 {
                            let old_update_time = update_0.last_update_time;
                            let old_acceleration = update_0.acceleration;
                            let delta_time = new_update_time - old_update_time;
                            let added_velocity = (old_acceleration + new_acceleration)
                                * Dimensionless::new(0.5)
                                * delta_time;
                            Some(if let Some(update_1) = &update_0.update_1 {
                                let old_velocity = update_1.velocity;
                                let new_velocity = old_velocity + added_velocity;
                                let added_position = (old_velocity + new_velocity)
                                    * Dimensionless::new(0.5)
                                    * delta_time;
                                Update1 {
                                    velocity: new_velocity,
                                    update_2_position: Some(
                                        if let Some(old_position) = update_1.update_2_position {
                                            old_position + added_position
                                        } else {
                                            added_position
                                        },
                                    ),
                                }
                            } else {
                                Update1 {
                                    velocity: added_velocity,
                                    update_2_position: None,
                                }
                            })
                        } else {
                            None
                        },
                    });
                }
                Ok(None) => {}
                Err(error) => {
                    self.update_0 = None;
                    return Err(error);
                }
            }
            Ok(())
        }
    }
}
pub use velocity_to_state::*;
mod velocity_to_state {
    use super::*;
    struct Update0 {
        last_update_time: Time,
        velocity: MillimeterPerSecond<f32>,
        update_1: Option<Update1>,
    }
    struct Update1 {
        position: Millimeter<f32>,
        acceleration: MillimeterPerSecondSquared<f32>,
    }
    ///Integrates and takes the derivative of a velocity to create a full [`State`] object. Uses
    ///trapezoidal integration.
    pub struct VelocityToState<G> {
        input: G,
        update_0: Option<Update0>,
    }
    impl<G> VelocityToState<G> {
        ///Constructor for `VelocityToState`.
        pub const fn new(input: G) -> Self {
            Self {
                input,
                update_0: None,
            }
        }
    }
    impl<G, E: Clone + Debug> Getter<State, E> for VelocityToState<G>
    where
        Self: Updatable<E>,
    {
        fn get(&self) -> Output<State, E> {
            if let Some(update_0) = &self.update_0 {
                if let Some(update_1) = &update_0.update_1 {
                    return Ok(Some(Datum::new(
                        update_0.last_update_time,
                        State::new(update_1.position, update_0.velocity, update_1.acceleration),
                    )));
                }
            }
            Ok(None)
        }
    }
    impl<G: Getter<MillimeterPerSecond<f32>, E>, E: Clone + Debug> Updatable<E> for VelocityToState<G> {
        fn update(&mut self) -> NothingOrError<E> {
            self.input.update()?;
            match self.input.get() {
                Ok(Some(new_velocity_datum)) => {
                    let new_update_time = new_velocity_datum.time;
                    let new_velocity = new_velocity_datum.value;
                    self.update_0 = Some(Update0 {
                        last_update_time: new_update_time,
                        velocity: new_velocity,
                        update_1: if let Some(update_0) = &self.update_0 {
                            let old_update_time = update_0.last_update_time;
                            let old_velocity = update_0.velocity;
                            let delta_time = new_update_time - old_update_time;
                            let new_acceleration = (new_velocity - old_velocity) / delta_time;
                            let added_position = (old_velocity + new_velocity)
                                * Dimensionless::new(0.5)
                                * delta_time;
                            Some(Update1 {
                                position: if let Some(update_1) = &update_0.update_1 {
                                    update_1.position + added_position
                                } else {
                                    added_position
                                },
                                acceleration: new_acceleration,
                            })
                        } else {
                            None
                        },
                    })
                }
                Ok(None) => {}
                Err(error) => {
                    self.update_0 = None;
                    return Err(error);
                }
            }
            Ok(())
        }
    }
}
pub use position_to_state::*;
mod position_to_state {
    use super::*;
    struct Update0 {
        last_update_time: Time,
        position: Millimeter<f32>,
        update_1: Option<Update1>,
    }
    struct Update1 {
        velocity: MillimeterPerSecond<f32>,
        update_2_acceleration: Option<MillimeterPerSecondSquared<f32>>,
    }
    ///Takes the second derivative of a position to create a full [`State`] object.
    pub struct PositionToState<G> {
        input: G,
        update_0: Option<Update0>,
    }
    impl<G> PositionToState<G> {
        ///Constructor for `PositionToState`.
        pub const fn new(input: G) -> Self {
            Self {
                input,
                update_0: None,
            }
        }
    }
    impl<G, E: Clone + Debug> Getter<State, E> for PositionToState<G>
    where
        Self: Updatable<E>,
    {
        fn get(&self) -> Output<State, E> {
            if let Some(update_0) = &self.update_0 {
                if let Some(update_1) = &update_0.update_1 {
                    if let Some(update_2_acceleration) = update_1.update_2_acceleration {
                        return Ok(Some(Datum::new(
                            update_0.last_update_time,
                            State::new(update_0.position, update_1.velocity, update_2_acceleration),
                        )));
                    }
                }
            }
            Ok(None)
        }
    }
    impl<G: Getter<Millimeter<f32>, E>, E: Clone + Debug> Updatable<E> for PositionToState<G> {
        fn update(&mut self) -> NothingOrError<E> {
            self.input.update()?;
            match self.input.get() {
                Ok(Some(new_position_datum)) => {
                    let new_update_time = new_position_datum.time;
                    let new_position = new_position_datum.value;
                    self.update_0 = Some(Update0 {
                        last_update_time: new_update_time,
                        position: new_position,
                        update_1: if let Some(update_0) = &self.update_0 {
                            let old_update_time = update_0.last_update_time;
                            let old_position = update_0.position;
                            let delta_time = new_update_time - old_update_time;
                            let new_velocity = (new_position - old_position) / delta_time;
                            Some(Update1 {
                                velocity: new_velocity,
                                update_2_acceleration: if let Some(update_1) = &update_0.update_1 {
                                    let old_velocity = update_1.velocity;
                                    let new_acceleration =
                                        (new_velocity - old_velocity) / delta_time;
                                    Some(new_acceleration)
                                } else {
                                    None
                                },
                            })
                        } else {
                            None
                        },
                    });
                }
                Ok(None) => {}
                Err(error) => {
                    self.update_0 = None;
                    return Err(error);
                }
            }
            Ok(())
        }
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
            input,
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
        self.input.update()?;
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
            input,
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
        self.input.update()?;
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
            input,
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
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        Ok(())
    }
}
///Converts errors returned by a getter to another type through [`Into`]. Leaves `Ok` values
///unchanged.
pub struct ErrorIntoConverter<T, G: Getter<T, EI>, EI: Clone + Debug> {
    input: G,
    phantom_t: PhantomData<T>,
    phantom_ei: PhantomData<EI>,
}
impl<T, G: Getter<T, EI>, EI: Clone + Debug> ErrorIntoConverter<T, G, EI> {
    ///Constructor for `ErrorIntoConverter`.
    pub const fn new(input: G) -> Self {
        Self {
            input,
            phantom_t: PhantomData,
            phantom_ei: PhantomData,
        }
    }
}
impl<T, G, EI, EO> Getter<T, EO> for ErrorIntoConverter<T, G, EI>
where
    G: Getter<T, EI>,
    EI: Clone + Debug + Into<EO>,
    EO: Clone + Debug,
{
    fn get(&self) -> Output<T, EO> {
        self.input.get().map_err(|error| error.into())
    }
}
impl<T, G: Getter<T, EI>, EI: Clone + Debug + Into<EO>, EO: Clone + Debug> Updatable<EO>
    for ErrorIntoConverter<T, G, EI>
{
    fn update(&mut self) -> NothingOrError<EO> {
        self.input.update().map_err(|error| error.into())?;
        Ok(())
    }
}
