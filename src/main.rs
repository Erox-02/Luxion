#![no_std]
#![no_main]

use esp32s3_hal::{
    gpio::{
        Gpio4, Gpio5, Gpio6, Gpio7, Gpio10, Gpio11, Gpio12, Gpio13,
        Gpio15, Gpio16, Gpio17, Gpio18, Gpio46,
        Input, Output, PushPull, IO,
    },
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi::{master::{Spi, SpiMode}, SpiDataMode},
    ledc::{channel, timer, Ledc, Resolution},
    syscon::SystemConfig,
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
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let systm = SystemConfig::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(systm.clock_control).execute();
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
    let _int = pins.gpio46.into_push_pull_input();

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
    let mut ledc_timer = ledc.timer::<timer::Number1, _>(
        timer::Lstm32,
        timer::source::APB_CLK,
        Resolution::Bits10,
    );
    ledc_timer.configure(&mut ledc);

    let channel0 = ledc.channel::<channel::Number0, _>(m1_ina);
    let channel1 = ledc.channel::<channel::Number1, _>(m1_inb);
    let channel2 = ledc.channel::<channel::Number2, _>(m2_ina);
    let channel3 = ledc.channel::<channel::Number3, _>(m2_inb);
    let channel4 = ledc.channel::<channel::Number4, _>(m3_ina);
    let channel5 = ledc.channel::<channel::Number5, _>(m3_inb);
    let channel6 = ledc.channel::<channel::Number6, _>(m4_ina);
    let channel7 = ledc.channel::<channel::Number7, _>(m4_inb);

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
    let throttle = 0.0;
    let target_roll = 0.0;
    let target_pitch = 0.0;
    let target_yaw = 0.0;
    let armed = false;

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

        //space left for esp-now for later
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

//place for esp-now 