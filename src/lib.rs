pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
}
impl PIDController{
    pub fn new(setpoint: f32,
               kp: f32,
               ki: f32,
               kd: f32)->PIDController{
        PIDController{
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0
        }
    }
    pub fn update(&mut self, time: f32, process: f32){
        let error = self.setpoint-process;
        let delta_time = time-self.last_update_time;
        let drv_error = (error-self.prev_error)/delta_time;
        self.int_error += delta_time*(self.prev_error+error)/2.0;
        (self.set_control)(self.kp*error+self.ki*self.int_error+self.kd*drv_error);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
    }
}
