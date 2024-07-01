///Coefficients for a PID controller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIDKValues {
    ///Proportional coefficient.
    pub kp: f32,
    ///Integral coefficient.
    pub ki: f32,
    ///Derivative coefficient.
    pub kd: f32,
}
impl PIDKValues {
    ///Constructor for `PIDKValues`.
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp: kp,
            ki: ki,
            kd: kd,
        }
    }
}
///A proportional-integral-derivative controller. This will probably be removed in the future and
///you should prefer `rrtk::streams::control::PIDControllerStream` instead.
pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<i64>,
    prev_error: Option<f32>,
    int_error: f32,
}
impl PIDController {
    ///Constructor for `PIDController`.
    pub fn new(setpoint: f32, kvalues: PIDKValues) -> Self {
        PIDController {
            setpoint: setpoint,
            kp: kvalues.kp,
            ki: kvalues.ki,
            kd: kvalues.kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
        }
    }
    ///Update the PID controller. Give it a new time and process variable value, and it will give
    ///you a new control variable value.
    #[must_use]
    pub fn update(&mut self, time: i64, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / (delta_time as f32),
        };
        self.int_error += match self.prev_error {
            Some(x) => (delta_time as f32) * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        self.kp * error + self.ki * self.int_error + self.kd * drv_error
    }
}
///A PID controller that will integrate the control variable a given number of times to simplify
///control of some systems such as motors. `N` is one more than the number of times it integrates.
///Do not set `N` to 0. This will probably be removed in the future and you should prefer
///`rrtk::streams::control::PIDControllerStream` instead. Use it as an input for a chain of
///`rrtk::streams::math::IntegralStream`s to recreate the shift behavior.
pub struct PIDControllerShift<const N: usize> {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<i64>,
    prev_error: Option<f32>,
    int_error: f32,
    shifts: [f32; N],
}
impl<const N: usize> PIDControllerShift<N> {
    ///Constructor for `PIDControllerShift`.
    pub fn new(setpoint: f32, kvalues: PIDKValues) -> Self {
        if N < 1 {
            panic!("PIDControllerShift N must be at least 1. N is one more than the number of times it integrates.")
        }
        Self {
            setpoint: setpoint,
            kp: kvalues.kp,
            ki: kvalues.ki,
            kd: kvalues.kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
            shifts: [0.0; N],
        }
    }
    ///Update the PID controller. Give it a new time and process variable value, and it will give
    ///you a new control variable value.
    #[must_use]
    pub fn update(&mut self, time: i64, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / (delta_time as f32),
        };
        self.int_error += match self.prev_error {
            Some(x) => (delta_time as f32) * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        let control = self.kp * error + self.ki * self.int_error + self.kd * drv_error;
        let mut new_shifts = [0.0; N];
        new_shifts[0] = control;
        for i in 1..N {
            let prev_int = self.shifts[i];
            new_shifts[i] =
                prev_int + (delta_time as f32) * (self.shifts[i - 1] + new_shifts[i - 1]) / 2.0;
        }
        self.shifts = new_shifts;
        self.shifts[self.shifts.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pid_new() {
        let pid = PIDController::new(5.0, PIDKValues::new(1.0, 0.01, 0.1));
        assert_eq!(pid.setpoint, 5.0);
        assert_eq!(pid.kp, 1.0);
        assert_eq!(pid.ki, 0.01);
        assert_eq!(pid.kd, 0.1);
        assert_eq!(pid.last_update_time, None);
        assert_eq!(pid.prev_error, None);
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn pid_initial_update() {
        let mut pid = PIDController::new(5.0, PIDKValues::new(1.0, 0.01, 0.1));
        let new_control = pid.update(1, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn pid_subsequent_update() {
        let mut pid = PIDController::new(5.0, PIDKValues::new(1.0, 0.01, 0.1));
        let _ = pid.update(1, 0.0);
        let new_control = pid.update(3, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    fn pidshift_no_shift() {
        let mut pid = PIDControllerShift::<1>::new(5.0, PIDKValues::new(1.0, 0.01, 0.1));
        let _ = pid.update(1, 0.0);
        let new_control = pid.update(3, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, [4.04]);
    }
}
