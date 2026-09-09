use embedded_hal::spi::SpiBus;

pub const WHO_AM_I: u8 = 0x75;
pub const PWR_MGMT_1: u8 = 0x6B;
pub const PWR_MGMT_2: u8 = 0x6C;
pub const ACCEL_XOUT_H: u8 = 0x3B;
pub const ACCEL_XOUT_L: u8 = 0x3C;
pub const ACCEL_YOUT_H: u8 = 0x3D;
pub const ACCEL_YOUT_L: u8 = 0x3E;
pub const ACCEL_ZOUT_H: u8 = 0x3F;
pub const ACCEL_ZOUT_L: u8 = 0x40;
pub const GYRO_XOUT_H: u8 = 0x43;
pub const GYRO_XOUT_L: u8 = 0x44;
pub const GYRO_YOUT_H: u8 = 0x45;
pub const GYRO_YOUT_L: u8 = 0x46;
pub const GYRO_ZOUT_H: u8 = 0x47;
pub const GYRO_ZOUT_L: u8 = 0x48;
pub const TEMP_OUT_H: u8 = 0x41;
pub const TEMP_OUT_L: u8 = 0x42; //straight up cp and pasted the resisters lol

#[derive(Debug)]
pub enum Mpuerr {
    Spierr,
    WrnDev(u8),
    Unready, //the useless guy 
}

pub struct Mpu6500<SPI, CS> {
    spi: SPI,
    cs: CS,
}

#[derive(Clone, Copy, Debug)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub temp: f32,
}

impl <SPI, CS> Mpu6500<SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: embedded_hal::digital::OutputPin,
{  
    pub fn new(mut spi: SPI, mut cs: CS) -> Result<Self, Mpuerr> {
        let mut mpu = Mpu6500 {spi, cs};
        mpu.write_reg(PWR_MGMT_1, 0x80)?;
        esp32s3_hal::delay::Ets::delay_ms(100);
        mpu.write_reg(PWR_MGMT_1, 0x01)?;
        esp32s3_hal::delay::Ets::delay_ms(10);
        mpu.write_reg(0x1B, 0x00)?;
        mpu.write_reg(0x1C, 0x00)?;
        let who_am_i = mpu.read_reg(WHO_AM_I)?;
        if who_am_i != 0x70 && who_am_i != 0x73 {
            return Err(Mpuerr::WrnDev(who_am_i));
        }
        
        Ok(mpu)
    }

        pub fn read_data(&mut self) -> Result<ImuData, Mpuerr> {
        let mut buffer = [0u8; 14];
        self.read_bytes(ACCEL_XOUT_H, &mut buffer)?;
        
        let accel_x = (i16::from_be_bytes([buffer[0], buffer[1]]) as f32) / 16384.0;
        let accel_y = (i16::from_be_bytes([buffer[2], buffer[3]]) as f32) / 16384.0;
        let accel_z = (i16::from_be_bytes([buffer[4], buffer[5]]) as f32) / 16384.0;
                let temp_raw = i16::from_be_bytes([buffer[6], buffer[7]]);
        let temp = temp_raw as f32 / 333.87 + 21.0;
        
        let gyro_x = (i16::from_be_bytes([buffer[8], buffer[9]]) as f32) / 131.0;
        let gyro_y = (i16::from_be_bytes([buffer[10], buffer[11]]) as f32) / 131.0;
        let gyro_z = (i16::from_be_bytes([buffer[12], buffer[13]]) as f32) / 131.0;
        
        Ok(ImuData {
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            temp,
        })
    }

        pub fn read_reg(&mut self, reg: u8) -> Result<u8, Mpuerr> {
        let mut buffer = [0u8; 1];
        self.read_bytes(reg, &mut buffer)?;
        Ok(buffer[0])
    }

        pub fn read_bytes(&mut self, reg: u8, buffer: &mut [u8]) -> Result<(), Mpuerr> {
        self.cs.set_low().map_err(|_| Mpuerr::Spierr)?;
        
        let cmd = reg | 0x80;
        self.spi.write(&[cmd]).map_err(|_| Mpuerr::Spierr)?;
        
        for byte in buffer.iter_mut() {
            *byte = self.spi.read().map_err(|_| Mpuerr::Spierr)?;
        }
        
        self.cs.set_high().map_err(|_| Mpuerr::Spierr)?;
        Ok(())
    }

    pub fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), Mpuerr> {
        self.cs.set_low().map_err(|_| Mpuerr::Spierr)?;
        
        let cmd = reg & 0x7F;
        self.spi.write(&[cmd, value]).map_err(|_| Mpuerr::Spierr)?;
        
        self.cs.set_high().map_err(|_| Mpuerr::Spierr)?;
        Ok(())
    }
}

     
