use esp32s3_hal::ledc::{Ledc, channel::Channel, timer::Timer};

pub struct MotorCtrl<C1, C2 ,C3 ,C4, C5, C6 ,C7, C8> {
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
    C1: Channel<u16>,
    C2: Channel<u16>,
    C3: Chanenl<u16>,
    C4: Channel<u16>,
    C5: Channel<u16>,
    C6: Channel<u16>,
    C7: Channel<u16>,
    C8: Channel<u16>,
{
    pub fn new(
        ch1: C1, ch2: C2, ch3: C3, ch4: C4,
        ch5: C5, ch6: C6, ch7: C7, ch8: C8,
    ) -> Self {
        Self {
            ch1, ch2, ch3, ch4,
            ch5, ch6, ch7, ch8,
        }
    }

    pub fn set_motor(&mut self, motor: u8, speed:u16, cw:bool) {
        let speed = speed.min(1000);
        let (ina, inb) = match motor {
            0 => (&mut self.ch1, &mut self.ch2),
            1 => (&mut self.ch3, &mut self.ch4),
            2 => (&mut self.ch5, &mut self.ch6),
            3 => (&mut self.ch7, &mut self.ch8),
            _ => return,
        };

        if cw {
            ina.set_duty(speed);
            inb.set_duty(0);
        } else {
            ina.set_duty(0);
            inb.set_duty(speed);
        }
    }

        pub fn stop_all(&mut self) {
        self.ch1.set_duty(0);
        self.ch2.set_duty(0);
        self.ch3.set_duty(0);
        self.ch4.set_duty(0);
        self.ch5.set_duty(0);
        self.ch6.set_duty(0);
        self.ch7.set_duty(0);
        self.ch8.set_duty(0);
    }
}
    