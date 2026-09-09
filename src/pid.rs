#[derive(Clone, Copy)]
pub struct Pidctrl {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    prev_error: f32,
    integral_limit: f32,
    output_limit: f32,
}

impl Pidctrl {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            integral_limit: 100.0,
            output_limit: 100.0,
        }
    }
    
    pub fn with_limits(mut self, integral_limit: f32, output_limit: f32) -> Self {
        self.integral_limit = integral_limit;
        self.output_limit = output_limit;
        self
    }
    
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }
    
    pub fn calculate(&mut self, setpoint: f32, measurement: f32) -> f32 {
        let dt = 0.002; 
        
        
        let error = setpoint - measurement;
        let p_term = self.kp * error;
        
        self.integral += error * dt;
        self.integral = self.integral.clamp(-self.integral_limit, self.integral_limit);
        let i_term = self.ki * self.integral;
        
        let derivative = (error - self.prev_error) / dt;
        let d_term = self.kd * derivative;
        
        self.prev_error = error;
        
        let output = p_term + i_term + d_term;
        output.clamp(-self.output_limit, self.output_limit)
    }
}