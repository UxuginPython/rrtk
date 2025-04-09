# Changes
## 0.1.0
Initial release.
## 0.1.1
Fix motion profile issue.
## 0.2.0-alpha.1
Start new motor-encoder system.
## 0.2.0-alpha.2
Function for motors to follow motion profiles.
## 0.2.0-beta.1
Allow the user to run a custom update loop for motion profile following as an alternative to the single function.
## 0.2.0-beta.2
Add an update method to encoders.
## 0.2.0
Add an update method to motors, allow easier detection of parts of motion profiles, and reorganize the package to use features with the motor-encoder system in a module.
## 0.3.0-alpha.1
Start new stream system.
## 0.3.0-alpha.2
Reorganize a bit and add [EWMA](https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm) stream.
## 0.3.0-alpha.3
Add moving average stream.
## 0.3.0-alpha.4
- performance improvements
    - use array instead of vec for inputs to `SumStream` and `ProductStream`
    - avoid unnecessary weight sum calculation in `MovingAverageStream`
    - make the number of shifts in `PIDControllerShift` a constant
- replace all instances of `MotionProfileState` with `MotionProfilePiece`
- add `History` trait, which is like a `Stream` but you specify a time when you `get`
- reorganize streams into modules
- remove unnecessary `std` requirement for a couple types
## 0.3.0-alpha.5
- Move from `Stream` and the previous device system to `Getter` and `Settable`. `Getter` is like a stream or encoder and `Settable` is like a writable device.
- Add `Device` type which makes raw `Getter`s and `Settable`s work together better as mechanical devices in a system. This should represent a physical device.
- Add `Axle` type which contains multiple `Device` objects. It uses the capabilities of each device to control the real-life system. Eg. Data is gathered from `Getter` devices (`Device::Read` for encoders and `Device::ReadWrite` for servos) and used to control motors that do not contain their own control theory processing (`Device::ImpreciseWrite`), but motors that can do this on their own (`Device::ReadWrite` and `Device::PreciseWrite` depending on whether the internal data can be read) do not need this control. This object should represent a physical linkage between devices.
- Don't require a feature to be enabled for PID controller types
- Change API for PID controller types to be constructed with a k-values type rather than three individual `f32`s.
## 0.3.0-beta.1
- Don't require a feature to be enabled for motion profiles.
- Make `Settable` able to follow `Getter`s of the same type.
- Add `GetterFromHistory` struct allowing `History` objects to be used as `Getter`s.
## 0.3.0
- Add `set_delta` and `set_time` methods to `GetterFromHistory`.
- Move `streams::Constant` to `ConstantGetter`.
- Implement `Settable` for `ConstantGetter`.
- Add `get_last_request` method to `Settable`.
- Move `MotionProfile` `get_*` methods to `Option` instead of `Result`.
- Rename `UpdateOutput` to `NothingOrError`.
- Fix `Axle` bug where it would try to use nonexistent PID controllers for `Device::ImpreciseWrite` objects if it had not yet received a `Command`.
- Instead of directly implementing `set` in `Settable`, you now implement `direct_set`. You should still *call* just `set` though. This is a workaround required to make `SettableData` and `get_last_request` work correctly.
- Move `MotionProfile` to `History<Command, E>` instead of `History<State, E>`.
- Move timestamps to `i64` instread of `f32`. The recommended unit is nanoseconds. This is not `u64` due to the use of deltas.
- Fix `MovingAverageStream` panicing issue.
- Rename `StreamPID` to `PIDControllerStream`.
- Improve performance of `PIDControllerStream`.
- Mark `Error` enum as non-exhaustive.
- Write three example files.
- Derive additional traits for a few structs.
- Give `MotionProfile` a return value after it has completed. This is based on the end state provided to the constructor. It will choose the lowest possible position derivative to satisfy the end state. This means that if acceleration is 0, the position derivative in the command will be velocity, otherwise acceleration. If velocity is also 0, it will be position, otherwise just velocity.
- Add `get_(position|velocity|acceleration)` methods to `Command`.
- Add `Latest` stream allowing you to choose the output of whichever of a set of streams has the later timestamp.
- Implement `From<State>` for `Command`.
- Rename `TimeGetterFromStream` to `TimeGetterFromGetter`.
## 0.3.1
- Implement several `core::ops` traits and `Copy` for `State`
- Fix name of `PositionToState`
- Slightly improve performance of `MotionProfile` and `(Position|Velocity|Acceleration)ToState` by removing unnecessary code
- Improve tests
- Minor documentation fixes
- Add missing LGPL license notice to a few files
## 0.4.0-alpha.1
- Begin new device system.
## 0.4.0-alpha.2
- Make everything use `&RefCell<Terminal>` rather than `Rc<RefCell<Terminal>>`
- Make math streams use generics.
- Add `SettableCommandDeviceWrapper` and `GetterStateDeviceWrapper` allowing types only implementing `Settable<Command, _>` to be used as motors and types only implementing `Getter<State, _>` to be used as encoders.
- Revive `PositionDerivativeDependentPIDKValues`, now with a `get_k_values` method for getting the k-values for a specific position derivative.
- Add `evaluate` methods for `PIDKValues` and `PositionDerivativeDependentPIDKValues`.
- Add `CommandPID`, an easier and faster way to use PID control to turn a standard DC motor and an encoder into a de facto servo.
- Add `latest` function which gets the newer of two `Datum` objects.
## 0.4.0-alpha.3
- Add new `streams` submodules `flow` and `logic`.
- Add new streams
    - `Expirer`
    - `flow::IfStream`
    - `flow::IfElseStream`
    - `flow::FreezeStream`
    - `logic::AndStream`
    - `logic::OrStream`
    - `logic::NotStream`
- Pass through `Not` for `Datum<T>` where `T` implements `Not`.
- Add `NoneGetter`.
- Add `Axle` very similar to 0.4.0-alpha.1 one.
- Move `(SettableCommand|GetterState)DeviceWrapper` to `devices::wrappers` module.
- Add experimental `Device` implementor for a differential mechanism.
- Remove now-unused `GetterSettable` marker trait.
- Move new device system to a new `devices` feature.
- Minor documentation fix for `devices` module.
## 0.4.0-beta.1
- Make differential calculations able to trust all branches equally instead of ignoring one.
- Remove unnecessary `Box`ing from `InputGetter` and `InputTimeGetter`.
## 0.4.0-beta.2
- Rename `following_update` to `update_following_data` and remove `update` calls from it.
- Make `GetterFromHistory` use `&mut dyn History` instead of `Box<dyn History>` and make its constructors take `impl History` instead of `dyn History`.
- Remove now-unnecessary `new_for_motion_profile` constructor for `GetterFromHistory`.
- Remove `Clone` bound on `History<T, _>`'s `T`.
- Make `GetterFromHistory` return the requested timestamp as its `Datum` timestamp rather than that that the internal `History` returns.
- Make `make_input_getter` and `make_input_time_getter` functions instead of macros.
- Add `NoneGetter` constructor. (It is a unit struct, so this is redundant.);
- Add a `disconnect` method to `Terminal`.
- Add methods to builtin devices for getting references to their terminals.
- Slightly improve performance of `Terminal`'s `get` implementation by using an array of `MaybeUninit` rather than `Vec`.
- Minor documentation fixes.
## 0.4.0
- Fix `Invert` `get_terminal_2` which was returning terminal 1.
- Make terminals pass commands to their connected counterparts.
- Rename `SettableCommandDeviceWrapper` to `ActuatorWrapper`.
- Make `ActuatorWrapper` update its inner `Settable`.
- Make `ActuatorWrapper` call `update_terminals` in its `Updatable` implementation.
- Fix `CommandPID` error integral doubling bug.
- Add `TerminalData` type containing a timestamp, an optional command, and an optional state.
- Implement `Getter<TerminalData, _>` for `Terminal`.
- Add `PIDWrapper`, a  wrapper very similar to `ActuatorWrapper` that uses a `CommandPID` to control a DC motor rather than needing a servo or a control system set up by the user.
- Implement `TimeGetter` for `i64`. It will always return its value as a time.
- Remove unused `CannotConnectTerminals` error variant.
- Make `GetterStateDeviceWrapper` update its inner `Getter`.
- Keep `CommandPID` from resetting itself whenever it gets a new command rather than only when the command is different.
- Mark constructors for `State`, `Datum`, `PIDKValues`, `PositionDerivativeDependentPIDKValues`, and `Command` as `const`.
- Documentation improvements.
## 0.5.0-alpha.1
- Make a new `Reference` type that can hold a `*mut T`, `Rc<RefCell<T>>`, or `*const RwLock<T>`, allowing you to not need a dynamic allocator.
- Add `alloc` feature.
- Temporarily remove `devices::wrappers::PIDWrapper`. It will be back by the time this is stable.
## 0.5.0-alpha.2
- Move to BSD 3-Clause license.
- Implement `Clone` for `Reference`
- Add `to_dyn!` macro for creating `Reference<dyn Trait>` objects.
- Add function `rc_refcell_reference` and macros `static_reference!` and `static_rwlock_reference!` which work similarly to the former `make_input_getter`. They put their input into a container if required and then return a `Reference` to it.
- Make all stream inputs `?Sized`. This allows the use of `Reference<dyn Getter<_, _>>` and `Reference<dyn TimeGetter<_, _>>` in the builtin streams.
- Add `PIDWrapper` back.
- Update many tests to use `Reference`.
- Minor documentation changes.
## 0.5.0-beta.1
- Add `ArcRwLock`, `PtrMutex`, and `ArcMutex` `Reference` variants.
- Standardize snake_case of `ref_cell` and `rw_lock`.
- Standardize that the outermost container comes first in variable and function names: a `*const RwLock` is `ptr_rw_lock`, not `rw_lock_ptr`.
## 0.5.0
- Fix the potential for undefined behavior without an unsafe block by directly constructing `Reference` variants.
  - Rename `Reference` to `ReferenceUnsafe`.
    - Make `borrow` and `borrow_mut` methods of `ReferenceUnsafe` unsafe.
  - Add a wrapper struct for `ReferenceUnsafe` under the name `Reference`.
    - `Reference` (the wrapper struct) cannot be constructed with a raw pointer without an unsafe block or a macro that ensures that the pointer's target is static.
    - `Reference` has all of the same methods as `ReferenceUnsafe` except that `borrow` and `borrow_mut` are safe.
    - `Reference` has one additional method, `into_inner`, which returns its inner `ReferenceUnsafe`.
- Rewrite `SumStream` and `ProductStream` to not require `alloc`.
- Change macro scoping to allow both `rrtk::reference::to_dyn!` and `rrtk::to_dyn!` as valid paths, and similar scoping for other `Reference`-related macros. See the [documentation](https://docs.rs/rrtk/0.5.0) for more information.
- Derive `Eq` for `Datum`.
- Documentation improvements.
## 0.6.0-alpha.0
- Begin new dimensional analysis system.
## 0.6.0-alpha.1
- Use correct units in `Mul<Time>` and `Div<Time>` implementations for `Quantity`.
- Move constant `Unit`s to the `dimensions::constants` module, all of the items of which are reexported both to the `dimensions` module and at the crate's top level.
- Add many new constant units in addition to the original 6.
## 0.6.0-beta.0
- Add `dim_check_debug` and `dim_check_release` features.
- Document feature flags in crate-level documentation.
## 0.6.0
- Add `FloatToQuantity` and `QuantityToFloat` streams.
- Add `Sum2` and `Product2` streams, which are optimized for adding or multiplying two inputs faster than `SumStream` and `ProductStream`, which take any number of inputs.
- Implement:
  - `AddAssign` for `Time`
  - `SubAssign` for `Time`
  - `MulAssign<DimensionlessInteger>` for `Time`
  - `DivAssign<DimensionlessInteger>` for `Time`
  - `Add<Quantity>` for `Time`
  - `Sub<Quantity>` for `Time`
  - `Mul<Quantity>` for `Time`
  - `Div<Quantity>` for `Time`
  - `AddAssign` for `DimensionlessInteger`
  - `SubAssign` for `DimensionlessInteger`
  - `MulAssign` for `DimensionlessInteger`
  - `DivAssign` for `DimensionlessInteger`
  - `Add<Quantity>` for `DimensionlessInteger`
  - `Sub<Quantity>` for `DimensionlessInteger`
  - `Mul<Quantity>` for `DimensionlessInteger`
  - `Div<Quantity>` for `DimensionlessInteger`
  - `AddAssign` for `Quantity`
  - `SubAssign` for `Quantity`
  - `MulAssign` for `Quantity`
  - `DivAssign` for `Quantity`
  - `Add<Time>` for `Quantity`
  - `Sub<Time>` for `Quantity`
  - `AddAssign<Time>` for `Quantity`
  - `SubAssign<Time>` for `Quantity`
  - `MulAssign<Time>` for `Quantity`
  - `DivAssign<Time>` for `Quantity`
  - `Add<DimensionlessInteger>` for `Quantity`
  - `Sub<DimensionlessInteger>` for `Quantity`
  - `Mul<DimensionlessInteger>` for `Quantity`
  - `Div<DimensionlessInteger>` for `Quantity`
  - `AddAssign<DimensionlessInteger>` for `Quantity`
  - `SubAssign<DimensionlessInteger>` for `Quantity`
  - `MulAssign<DimensionlessInteger>` for `Quantity`
  - `DivAssign<DimensionlessInteger>` for `Quantity`
  - `AddAssign` for `Unit`
  - `SubAssign` for `Unit`
  - `MulAssign` for `Unit`
  - `DivAssign` for `Unit`
  - `Neg` for `Unit`
- Make `State::update` take `Time`.
- Make `State::set_constant_(position|velocity|acceleration)` take `Quantity`.
- Add `State::set_constant_(position|velocity|acceleration)_raw` functions to still allow setting each position derivative with `f32`.
- Make `State::new` take `Quantity` for position, velocity, and acceleration.
- Add `State::new_raw` to still allow constructing `State` with `f32` values.
- Make `(Position|Velocity|Acceleration)ToState` take `Quantity`.
- Make `IntegralStream` and `DerivativeStream` take `Quantity`.
- Make `EWMAStream` more generic, allowing it to take both `f32` and `Quantity`.
- Make `MovingAverageStream` more generic, allowing it to take both `f32` and `Quantity`.
- Mark `State::set_constant_(position|velocity|acceleration)` and their "raw" equivalents as const fn.
- Fix bug where the implementation of `From<PositionDerivative> for Unit` would return an incorrect second exponent.
- Fix unit issue in `MovingAverageStream`.
- Example improvements.
- Unit testing improvements.
- Documentation improvements.
## 0.6.1
- Add optional support for [`libm`](https://crates.io/crates/libm) and [`micromath`](https://crates.io/crates/micromath) for `no_std` float computation. Both are disabled by default.
- Propagate commands from terminals in `Axle` and `Invert` devices. (It is not possible in `Differential`).
- Add `GearTrain` device which also propagates commands.
- Add `replace_if_older_than` method to `Datum`.
- Add `OptionDatumExt` trait used for adding `replace_if_none_or_older_than` and `replace_if_none_or_older_than_option` methods to `Option<Datum<T>>`.
- Implement `Neg` for `Command`.
- Implement `Add`, `Sub`, `Mul<f32>`, and `Div<f32>` and their respective `*Assign` traits for `Command`.
- Implement `Getter<Command, E>` for `Terminal`.
- Minor documentation fixes.
## 0.7.0-alpha.0
- Begin new compile-time dimensional analysis system using a custom compile-time integer system.
- Somewhat rework `Time` to be more intuitive and to work more nicely with other types.
- Make some streams more generic in the numeric types they accept, specifically `IntegralStream` and `DerivativeStream`.
- Allow `Reference` to pass through `Getter` and `Updatable` implementations when its target implements them.
- `#[derive(Debug)]` for some `Reference`-related types.
- Minor documentation improvements.
## 0.7.0-alpha.1
Allow the use of `Getter` implementors to be used directly as stream inputs instead of needing to be in a `Reference`. One can, of course, still put stream inputs in `Reference` (as is necessary when using the same `Getter` in multiple places) since `Reference` now passes through the `Getter`, `Updatable`, and `TimeGetter` implementations of its referent.
## 0.7.0-alpha.2
Remove `Error` enum:
- Change `Output` type alias (`Getter::get`'s return type) from `Result<Option<Datum<T>>, Error<E>>` to `Result<Option<Datum<T>>, E>`.
- Change `TimeOutput` type alias (`TimeGetter::get`'s return type) from `Result<Time, Error<E>>` to `Result<Time, E>`.
- Change `NothingOrError` type alias (`Updatable::update`'s return type) from `Result<(), Error<E>>` to `Result<(), E>`.
- Make `NoneToError` and `TimeGetterFromGetter` require error values to return when they receive `Ok(None)`.
  - `TimeGetterFromGetter` no longer uses `NoneToError` internally, so remove its `T: Clone` bound.
