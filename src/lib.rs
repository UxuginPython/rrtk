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
        self.shifts[self.shifts.len()-1]
    }
}
pub struct State {
    position: f32,
    velocity: f32,
    acceleration: f32,
}
impl State {
    pub fn new(position: f32, velocity: f32, acceleration: f32) -> State {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let new_velocity = self.velocity + delta_time * self.acceleration;
        let new_position = self.position + delta_time * (self.velocity + new_velocity) / 2.0;
        self.position = new_position;
        self.velocity = new_velocity;
    }
    pub fn set_constant_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    pub fn set_constant_velocity(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    pub fn set_constant_position(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
}
/*If you are using a position-based encoder, ensure that it sums full rotations instead of
resetting to zero.*/
pub struct Encoder {
    state: State,
    time: f32,
}
impl Encoder {
    pub fn new(state: State, time: f32) -> Encoder {
        Encoder {
            state: state,
            time: time,
        }
    }
    pub fn update_acceleration(&mut self, time: f32, acceleration: f32) {
        let delta_time = time - self.time;
        let velocity = self.state.velocity + delta_time * (self.state.acceleration + acceleration) / 2.0;
        let position = self.state.position + delta_time * (self.state.velocity + velocity) / 2.0;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
    pub fn update_velocity(&mut self, time: f32, velocity: f32) {
        let delta_time = time - self.time;
        let acceleration = (velocity - self.state.velocity) / delta_time;
        let position = self.state.position + delta_time * (self.state.velocity + velocity) / 2.0;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
    pub fn update_position(&mut self, time: f32, position: f32) {
        let delta_time = time - self.time;
        let velocity = (position - self.state.position) / delta_time;
        let acceleration = (velocity - self.state.velocity) / delta_time;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
}
pub enum MotorMode {
    POSITION,
    VELOCITY,
    ACCELERATION,
}
pub struct Motor {
    encoder: Encoder,
    pid: PIDControllerShift,
    mode: MotorMode,
}
impl Motor {
    pub fn new(state: State, time: f32, mode: MotorMode, setpoint: f32) -> Motor {
        Motor {
            encoder: Encoder::new(state, time),
            pid: PIDControllerShift::new(setpoint, 1.0, 0.01, 0.1, match mode {
                MotorMode::POSITION => 0,
                MotorMode::VELOCITY => 1,
                MotorMode::ACCELERATION => 2,
            }),
            mode: mode,
        }
    }
    pub fn set_constant(&mut self, mode: MotorMode, setpoint: f32) {
        self.mode = mode;
        self.pid = PIDControllerShift::new(setpoint, 1.0, 0.01, 0.1, match self.mode {
            MotorMode::POSITION => 0,
            MotorMode::VELOCITY => 1,
            MotorMode::ACCELERATION => 2,
        });
    }
    /*The recommended way of doing this is
    time = get_time();
    velocity = get_velocity();
    motor.encoder.update_velocity(time, velocity);
    run_motor_at_voltage(motor.update(time));
    (API will differ.)*/
    /*The reason the encoder is not updated with the motor update method
    is to allow for encoders reporting different metrics, as there are both
    velocity- and position-based encoders.*/
    #[must_use]
    pub fn update(&mut self, time: f32) -> f32 {
        self.pid.update(time, match &self.mode {
            MotorMode::POSITION => self.encoder.state.position,
            MotorMode::VELOCITY => self.encoder.state.velocity,
            MotorMode::ACCELERATION => self.encoder.state.acceleration,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pid_new() {
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
    fn pid_initial_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let new_control = pid.update(1.0, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1.0));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn pid_subsequent_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    fn pidshift_no_shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 0);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, vec![4.04]);
    }
    #[test]
    fn pidshift_shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 9.04);
    }
    #[test]
    fn state_new() {
        let state = State::new(1.0, 2.0, 3.0);
        assert_eq!(state.position, 1.0);
        assert_eq!(state.velocity, 2.0);
        assert_eq!(state.acceleration, 3.0);
    }
    #[test]
    fn state_update() {
        let mut state = State::new(1.0, 2.0, 3.0);
        state.update(4.0);
        assert_eq!(state.position, 33.0);
        assert_eq!(state.velocity, 14.0);
        assert_eq!(state.acceleration, 3.0);
    }
    #[test]
    fn state_acceleration() {
        let mut state = State::new(1.0, 2.0, 3.0);
        state.set_constant_acceleration(4.0);
        assert_eq!(state.acceleration, 4.0);
    }
    #[test]
    fn state_velocity() {
        let mut state = State::new(1.0, 2.0, 3.0);
        state.set_constant_velocity(4.0);
        assert_eq!(state.velocity, 4.0);
        assert_eq!(state.acceleration, 0.0);
    }
    #[test]
    fn state_position() {
        let mut state = State::new(1.0, 2.0, 3.0);
        state.set_constant_position(4.0);
        assert_eq!(state.position, 4.0);
        assert_eq!(state.velocity, 0.0);
        assert_eq!(state.acceleration, 0.0);
    }
    #[test]
    fn encoder_new() {
        let encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
        assert_eq!(encoder.state.position, 1.0);
        assert_eq!(encoder.state.velocity, 2.0);
        assert_eq!(encoder.state.acceleration, 3.0);
        assert_eq!(encoder.time, 4.0);
    }
    #[test]
    fn encoder_update_acceleration() {
        let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
        encoder.update_acceleration(6.0, 5.0);
        assert_eq!(encoder.state.position, 13.0);
        assert_eq!(encoder.state.velocity, 10.0);
        assert_eq!(encoder.state.acceleration, 5.0);
    }
    #[test]
    fn encoder_update_velocity() {
        let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
        encoder.update_velocity(6.0, 5.0);
        assert_eq!(encoder.state.position, 8.0);
        assert_eq!(encoder.state.velocity, 5.0);
        assert_eq!(encoder.state.acceleration, 1.5);
    }
    #[test]
    fn encoder_update_position() {
        let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
        encoder.update_position(6.0, 5.0);
        assert_eq!(encoder.state.position, 5.0);
        assert_eq!(encoder.state.velocity, 2.0);
        assert_eq!(encoder.state.acceleration, 0.0);
    }
    #[test]
    fn motor_new() {
        let motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        assert_eq!(motor.encoder.state.position, 1.0);
        assert_eq!(motor.encoder.state.velocity, 2.0);
        assert_eq!(motor.encoder.state.acceleration, 3.0);
        assert_eq!(motor.encoder.time, 4.0);
        assert_eq!(motor.pid.setpoint, 3.0);
        assert_eq!(motor.pid.kp, 1.0);
        assert_eq!(motor.pid.ki, 0.01);
        assert_eq!(motor.pid.kd, 0.1);
        assert_eq!(motor.pid.shifts.len(), 3);
    }
    #[test]
    fn motor_set_constant() {
        let mut motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        motor.set_constant(MotorMode::VELOCITY, 5.0);
        assert_eq!(motor.pid.shifts.len(), 2);
        assert_eq!(motor.pid.setpoint, 5.0);
    }
}
