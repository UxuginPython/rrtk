// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Traits making it easier to set up common devices that cannot be builtin structs.
use crate::*;
///Internal data needed by `ImpreciseMotor` implementors.
pub struct ImpreciseMotorData<E: Copy + Debug> {
    terminal: Rc<RefCell<Terminal<E>>>,
}
///A motor without a builtin encoder.
pub trait ImpreciseMotor<E: Copy + Debug> {
    ///Get an immutable reference to the object's `ImpreciseMotorData`.
    fn get_imprecise_motor_data_ref(&self) -> &ImpreciseMotorData<E>;
    ///Get a mutable reference to the object's `ImpreciseMotorData`.
    fn get_imprecise_motor_data_mut(&mut self) -> &mut ImpreciseMotorData<E>;
    ///Set the voltage of the motor or something proportional to it.
    fn set_voltage(&mut self, voltage: f32);
}
impl<E: Copy + Debug> Device<E> for dyn ImpreciseMotor<E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        let data = self.get_imprecise_motor_data_mut();
        data.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<E: Copy + Debug> Updatable<E> for dyn ImpreciseMotor<E> {
    fn update(&mut self) -> NothingOrError<E> {
        todo!();
    }
}
