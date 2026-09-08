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
}