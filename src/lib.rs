//I need to make a commit to merge this.
pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
}
impl PIDController {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32) -> PIDController {
        PIDController {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
        }
    }
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        self.kp * error + self.ki * self.int_error + self.kd * drv_error
    }
}
pub struct PIDControllerShift {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
    shifts: Vec<f32>,
}
impl PIDControllerShift {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32, shift: u8) -> PIDControllerShift {
        let mut shifts = Vec::new();
        for _ in 0..shift+1 {
            shifts.push(0.0);
        }
        PIDControllerShift {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
            shifts: shifts,
        }
    }
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        let control = self.kp * error + self.ki * self.int_error + self.kd * drv_error;
        let mut new_shifts = vec![control];
        for i in 1..self.shifts.len(){
            let prev_int = self.shifts[i];
            new_shifts.push(prev_int+delta_time*(self.shifts[i-1]+new_shifts[i-1])/2.0);
        }
        self.shifts=new_shifts;
        println!("{:#?}", self.shifts);
        self.shifts[self.shifts.len()-1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        assert_eq!(pid.setpoint, 5.0);
        assert_eq!(pid.kp, 1.0);
        assert_eq!(pid.ki, 0.01);
        assert_eq!(pid.kd, 0.1);
        assert_eq!(pid.last_update_time, None);
        assert_eq!(pid.prev_error, None);
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn initial_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let new_control = pid.update(1.0, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1.0));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn subsequent_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    fn no_shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 0);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, vec![4.04]);
    }
    #[test]
    fn shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 9.04);
    }
}
