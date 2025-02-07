#![no_std]
#![no_main]

use bh1750::bh1750::{Bh1750, Command};
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_println::println;
use esp_hal::gpio::{Level, Pull, OutputOpenDrain};

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    const ADDRESS: u8 = 0x23;

    let sda = OutputOpenDrain::new(_peripherals.GPIO8, Level::High, Pull::Up);
    let scl = OutputOpenDrain::new(_peripherals.GPIO9, Level::High, Pull::Up);


    let mut i2c = I2c::new(
        _peripherals.I2C0,
        Config {
            frequency: 400u32.kHz(),
            timeout: Some(1000),
        },
    ).with_scl(scl).with_sda(sda);

    let mut bh1750 = Bh1750::new(&mut i2c, ADDRESS);

    bh1750.power_off().unwrap();
    bh1750.power_on().unwrap();

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    loop {
        let data = bh1750.read(Command::OneTimeHResolutionMode).unwrap();
        println!("data: {:?}", data);
        delay.delay(500.millis());
    }
}
