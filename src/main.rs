#![no_std]
#![no_main]

use esp32s3_hal::{
    gpio::{Gpio4, Gpio5, Gpio6, Gpio7, Gpio10, Gpio11, Gpio12, Gpio13, Gpio15, Gpio16, Gpio17, Gpio18, Gpio46, Output, PushPull, IO},
    clock::ClockControl,    
    syscon::SystemConfig,
    pac::Peripherals,
    prelude::*,
    spi::{master::{Spi, SpiMode}, SpiDataMode},
    timer::TimerGroup,
    Rtc, Cpu,
    ledc::{channel, timer, Ledc, Resolution},
    syscon::Syscon,
};

use esp_println::println;
use esp_backtrace as _;

mod mpu;
mod pid:
mod motor;
mod filter;

use mpu::
use motor::;
use filter::;
use pid::;

type M1InA = Gpio4<Output<PushPull>>;
type M1InB = Gpio5<Output<PushPull>>;
type M2InA = Gpio6<Output<PushPull>>;
type M2InB = Gpio7<Output<PushPull>>;
type M3InA = Gpio15<Output<PushPull>>;
type M3InB = Gpio16<Output<PushPull>>;
type M4InA = Gpio17<Output<PushPull>>;
type M4InB = Gpio18<Output<PushPull>>;
type Cs = Gpio10<Output<PushPull>>;
type Sck = Gpio12<Output<PushPull>>;
type Mosi = Gpio11<Output<PushPull>>;
type Miso = Gpio13<Input<PushPull>>;
type Int = Gpio46<Input<PushPull>>; 

#[entry]
fn main() ->! {
    let peripherals = Peripherals::take().unwarp();
    let systm = SystemConfig::new(peripherals.SYSTEM);
    let clock = ClockControl::max(systm.clock_control).execute();
    let timer = TimerGroup::new(peripherals.TIMG0, &clocks);
    let rtc = Rtc::new(peripherals.RTC_CNTL);
    let cpu_ctrl = Cpu::new(peripherals.CPU_CTRL);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;
    let m1_ina = pins.gpio4.into_push_pull_output();
    let m1_inb = pins.gpio5.into_push_pull_output();
    let m2_ina = pins.gpio6.into_push_pull_output();
    let m2_inb = pins.gpio7.into_push_pull_output();
    let m3_ina = pins.gpio15.into_push_pull_output();
    let m3_inb = pins.gpio16.into_push_pull_output();
    let m4_ina = pins.gpio17.into_push_pull_output();
    let m4_inb = pins.gpio18.into_push_pull_output();  //m1 and m4 are cw

    let cs = pins.gpio10.into_push_pull_output();
    let sck = pins.gpio12.into_push_pull_output();
    let mosi = pins.gpio11.into_push_pull_output();
    let miso = pins.gpio13.into_push_pull_input();
    let int = pins.gpio46.into_push_pull_input();
    let spi = Spi::new(
        peripherals.SPI2,
        SpiMode::Mode0,
        SpiDataMode::FullDuplex,
        &clocks,
    )
    .with_mosi(mosi)
    .with_miso(miso)
    .with_sck(sck);

    let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    let mut timer = ledc.timer::<timer::Number1, _>(timer::Lstm32, timer::source::APB_CLK, Resolution::Bits10);
    timer.configure(&mut ledc);

    let mut channel1 = ledc.channel::<channel::Number1, _>(m1_ina);
    let mut channel2 = ledc.channel::<channel::Number2, _>(m1_inb);
    let mut channel3 = ledc.channel::<channel::Number3, _>(m2_ina);
    let mut channel4 = ledc.channel::<channel::Number4, _>(m2_inb);
    let mut channel5 = ledc.channel::<channel::Number5, _>(m3_ina);
    let mut channel6 = ledc.channel::<channel::Number6, _>(m3_inb);
    let mut channel7 = ledc.channel::<channel::Number7, _>(m4_ina);
    let mut channel8 = ledc.channel::<channel::Number8, _>(m4_inb);

    let mut motor_ctrl = MotorCtrl::new(
        chanel1, chanel2, chanel3, chanel4,
        chanel5, chanel6, chanel7, chanel8,
    );

    let mut mpu = match Mpu6500::new(spi, cs) {
        Ok(mpu) => {
            println!("imu init'ed");
            mpu
        }
        Err(e) => {
            loop {}
        }
    };
    let mut roll = Pidctrl::new(x, x, x);
    let mut pitch = Pidctrl::new(x, x, x);
    let mut yaw = Pidctrl::new(x, x, x);

    let mut filter = ComFil::new(x);
    let mut throttle = 0.0;
    let mut target_roll = 0.0;
    let mut target_pitch = 0.0;
    let mut target_yaw = 0.0;
    let mut armed = false;

    let loop_delay = 10; 
    let mut previous_time = 0u32;
    println!("fly");

    loop {
        match mpu.read_data() {
            Ok(data) => {
                let attitude = filter.update(data.accel_x, data.accel_y, data.accel_z, 
                                           data.gyro_x, data.gyro_y, data.gyro_z, 
                                           loop_delay as f32 / 1000.0);
                            
                let roll_output = pid_roll.calculate(attitude.roll, target_roll);
                let pitch_output = pid_pitch.calculate(attitude.pitch, target_pitch);
                let yaw_output = pid_yaw.calculate(attitude.yaw, target_yaw);
  
                if armed && throttle > 0.05 {
                    let motor_speeds = mix_motors(throttle, roll_output, pitch_output, yaw_output);
                    motor_ctrl.set_motor(0, motor_speeds[0], true);
                    motor_ctrl.set_motor(1, motor_speeds[1], false);
                    motor_ctrl.set_motor(2, motor_speeds[2], false);
                    motor_ctrl.set_motor(3, motor_speeds[3], true);
                } else {
                    motor_ctrl.stop_all();
                }
                } else {
                    motor_ctrl.stop_all();
                }
                
                println!("R:{:.2} P:{:.2} Y:{:.2} | T:{:.2}", 
                         attitude.roll, attitude.pitch, attitude.yaw, throttle);
            }
            Err(e) => {
                println!("imu read error , {:?}", e);
            }
        }

        //space left for esp-now for later 
}

pub fn mix_motors(throttle: f32, roll: f32, pitch: f32, yaw: f32) -> [u16; 4] {
    let m1 = (throttle - pitch + roll + yaw).clamp(0.0, 1.0) * 1000.0;
    let m2 = (throttle + pitch + roll - yaw).clamp(0.0, 1.0) * 1000.0;
    let m3 = (throttle + pitch - roll + yaw).clamp(0.0, 1.0) * 1000.0;
    let m4 = (throttle - pitch - roll - yaw).clamp(0.0, 1.0) * 1000.0;
    
    [m1 as u16, m2 as u16, m3 as u16, m4 as u16]
}

enum Command {
    Arm,
    Disarm,
    Throttle(f32),
    SetTarget(f32, f32, f32),
}

//place for esp-now 