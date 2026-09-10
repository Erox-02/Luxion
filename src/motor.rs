use esp32s3_hal::ledc::{
    channel::{Channel, ChannelIFace},
    timer::Timer,
    Ledc,
};

pub struct MotorCtrl<C1, C2, C3, C4, C5, C6, C7, C8> {
    ch1: C1,
    ch2: C2,
    ch3: C3,
    ch4: C4,
    ch5: C5,
    ch6: C6,
    ch7: C7,
    ch8: C8,
}

impl<C1, C2, C3, C4, C5, C6, C7, C8> MotorCtrl<C1, C2, C3, C4, C5, C6, C7, C8>
where
    C1: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C2: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C3: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C4: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C5: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C6: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C7: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
    C8: ChannelIFace<'static, esp32s3_hal::ledc::LowSpeed>,
{
    pub fn new(
        ch1: C1,
        ch2: C2,
        ch3: C3,
        ch4: C4,
        ch5: C5,
        ch6: C6,
        ch7: C7,
        ch8: C8,
    ) -> Self {
        Self {
            ch1,
            ch2,
            ch3,
            ch4,
            ch5,
            ch6,
            ch7,
            ch8,
        }
    }

    pub fn set_motor(&mut self, motor: u8, speed: u16, cw: bool) {
        let speed = speed.min(1000);
        let (ina, inb) = match motor {
            0 => (&mut self.ch1, &mut self.ch2),
            1 => (&mut self.ch3, &mut self.ch4),
            2 => (&mut self.ch5, &mut self.ch6),
            3 => (&mut self.ch7, &mut self.ch8),
            _ => return,
        };

        let duty = (speed as u32 * 255 / 1000) as u8;

        if cw {
            let _ = ina.set_duty(duty);
            let _ = inb.set_duty(0);
        } else {
            let _ = ina.set_duty(0);
            let _ = inb.set_duty(duty);
        }
    }

    pub fn stop_all(&mut self) {
        let _ = self.ch1.set_duty(0);
        let _ = self.ch2.set_duty(0);
        let _ = self.ch3.set_duty(0);
        let _ = self.ch4.set_duty(0);
        let _ = self.ch5.set_duty(0);
        let _ = self.ch6.set_duty(0);
        let _ = self.ch7.set_duty(0);
        let _ = self.ch8.set_duty(0);
    }
}