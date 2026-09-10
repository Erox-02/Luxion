#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    gpio::{Input, Output},
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    spi::{
        master::{Config as SpiConfig, Spi},
        Mode,
    },
    time::Rate,
};

use esp_println::println;
use esp_backtrace as _;

mod mpu;
mod pid;
mod motor;
mod filter;

use mpu::Mpu6500;
use motor::MotorCtrl;
use filter::ComFil;
use pid::Pidctrl;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let io = esp_hal::gpio::Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let m1_ina = Output::new(io.pins.gpio4, esp_hal::gpio::Level::Low);
    let m1_inb = Output::new(io.pins.gpio5, esp_hal::gpio::Level::Low);
    let m2_ina = Output::new(io.pins.gpio6, esp_hal::gpio::Level::Low);
    let m2_inb = Output::new(io.pins.gpio7, esp_hal::gpio::Level::Low);
    let m3_ina = Output::new(io.pins.gpio15, esp_hal::gpio::Level::Low);
    let m3_inb = Output::new(io.pins.gpio16, esp_hal::gpio::Level::Low);
    let m4_ina = Output::new(io.pins.gpio17, esp_hal::gpio::Level::Low);
    let m4_inb = Output::new(io.pins.gpio18, esp_hal::gpio::Level::Low);

    let cs = Output::new(io.pins.gpio10, esp_hal::gpio::Level::High);
    let sck = Output::new(io.pins.gpio12, esp_hal::gpio::Level::Low);
    let mosi = Output::new(io.pins.gpio11, esp_hal::gpio::Level::Low);
    let miso = Input::new(io.pins.gpio13);
    let _int = Input::new(io.pins.gpio46);

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(1))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sck)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_cs(cs);

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty10Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(20),
        })
        .unwrap();

    let channel0 = ledc.channel(channel::Number::Channel0, m1_ina);
    let channel1 = ledc.channel(channel::Number::Channel1, m1_inb);
    let channel2 = ledc.channel(channel::Number::Channel2, m2_ina);
    let channel3 = ledc.channel(channel::Number::Channel3, m2_inb);
    let channel4 = ledc.channel(channel::Number::Channel4, m3_ina);
    let channel5 = ledc.channel(channel::Number::Channel5, m3_inb);
    let channel6 = ledc.channel(channel::Number::Channel6, m4_ina);
    let channel7 = ledc.channel(channel::Number::Channel7, m4_inb);

    let mut motor_ctrl = MotorCtrl::new(
        channel0, channel1, channel2, channel3,
        channel4, channel5, channel6, channel7,
    );

    let mut mpu = match Mpu6500::new(spi, cs) {
        Ok(mpu) => {
            println!("imu init'ed");
            mpu
        }
        Err(e) => {
            loop {
                motor_ctrl.stop_all();
            }
        }
    };

    let mut roll = Pidctrl::new(1.0, 0.0, 0.0);
    let mut pitch = Pidctrl::new(1.0, 0.0, 0.0);
    let mut yaw = Pidctrl::new(1.0, 0.0, 0.0);

    let mut filter = ComFil::new(0.98);
    let mut throttle = 0.0;
    let mut target_roll = 0.0;
    let mut target_pitch = 0.0;
    let mut target_yaw = 0.0;
    let mut armed = false;

    let loop_delay = 10;
    let dt = loop_delay as f32 / 1000.0;

    println!("fly");

    loop {
        match mpu.read_data() {
            Ok(data) => {
                let attitude = filter.update(
                    data.accel_x,
                    data.accel_y,
                    data.accel_z,
                    data.gyro_x,
                    data.gyro_y,
                    data.gyro_z,
                    dt,
                );

                let roll_output = roll.calculate(target_roll, attitude.roll);
                let pitch_output = pitch.calculate(target_pitch, attitude.pitch);
                let yaw_output = yaw.calculate(target_yaw, attitude.yaw);

                if armed && throttle > 0.05 {
                    let motor_speeds =
                        mix_motors(throttle, roll_output, pitch_output, yaw_output);

                    motor_ctrl.set_motor(0, motor_speeds[0], true);
                    motor_ctrl.set_motor(1, motor_speeds[1], false);
                    motor_ctrl.set_motor(2, motor_speeds[2], false);
                    motor_ctrl.set_motor(3, motor_speeds[3], true);
                } else {
                    motor_ctrl.stop_all();
                }

                println!(
                    "R:{:.2} P:{:.2} Y:{:.2} | T:{:.2}",
                    attitude.roll,
                    attitude.pitch,
                    attitude.yaw,
                    throttle
                );
            }

            Err(e) => {
                println!("imu read error , {:?}", e);
                motor_ctrl.stop_all();
            }
        }
    }
}

pub fn mix_motors(
    throttle: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
) -> [u16; 4] {
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