#[derive(Clone, Copy)]

pub struct ComFil{
    alpha: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Attitude {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl ComFil {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn update(&mut self, ax: f32, ay: f32, az: f32, gx: f32, gy: f32, gz: f32, dt: f32) -> Attitude {
        let accel_roll = libm::atan2f(ay, az) * 180.0 / core::f32::consts::PI;
        let accel_pitch = libm::atan2f(-ax, libm::sqrtf(ay * ay + az * az)) * 180.0 / core::f32::consts::PI;
        self.roll = self.alpha * (self.roll + gx * dt) + (1.0 - self.alpha) * accel_roll;
        self.pitch = self.alpha * (self.pitch + gy * dt) + (1.0 - self.alpha) * accel_pitch;
        self.yaw += gz * dt;
                while self.yaw > 180.0 {
            self.yaw -= 360.0;
        }
        while self.yaw < -180.0 {
            self.yaw += 360.0;
        }
        
        Attitude {
            roll: self.roll,
            pitch: self.pitch,
            yaw: self.yaw,
        }
    }
    
        pub fn reset(&mut self) {
        self.roll = 0.0;
        self.pitch = 0.0;
        self.yaw = 0.0;
    }
}