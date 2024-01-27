pub struct PIDController {
    setpoint: f32,
    get_process: fn()->f32,
    get_time: fn()->f32,
    set_control: fn(f32),
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: f32,
    prev_error: f32,
    int_error: f32,
}
impl PIDController{
    pub fn new(setpoint: f32,
               get_process: fn()->f32,
               get_time: fn()->f32,
               set_control: fn(f32),
               kp: f32,
               ki: f32,
               kd: f32)->PIDController{
        PIDController{
            setpoint: setpoint,
            get_process: get_process,
            get_time: get_time,
            set_control: set_control,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: get_time(),
            prev_error: (setpoint-get_process())/setpoint,
            int_error: 0.0
        }
    }
    pub fn update(&mut self){
        let value = (self.get_process)();
        let time = (self.get_time)();
        let error = (self.setpoint-value)/self.setpoint;
        let delta_time = time-self.last_update_time;
        let drv_error = (error-self.prev_error)/delta_time;
        self.int_error += delta_time*(self.prev_error+error)/2.0;
        (self.set_control)(self.kp*error+self.ki*self.int_error+self.kd*drv_error);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn new() {
        fn get_process()->f32{
            1.0
        }
        fn get_time()->f32{
            0.0
        }
        fn set_control(_: f32){}
        let pid = PIDController::new(1.0, get_process, get_time, set_control, 1.0, 0.0, 0.0);
    }*/
    #[test]
    fn update(){
        let mut velocity = 0f32;
        let mut position = 0f32;
        let mut time = 0f32;
        fn get_process()->f32{
            position
        }
        fn get_time()->f32{
            time
        }
        fn set_control(value: f32){
            time+=1;
            velocity+=value;
            position+=velocity;
        }
        let mut pid = PIDController::new(1f32, get_process, get_time, set_control, 1.0, 0.0, 0.0);
    }
}
