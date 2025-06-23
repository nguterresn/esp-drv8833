#![no_std]
#![no_main]

use esp_drv8833::Stepper;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut stepper = Stepper::new(
        peripherals.GPIO0,
        peripherals.GPIO1,
        peripherals.GPIO10,
        peripherals.GPIO9,
        Rate::from_hz(200),
        200,
    );

    let delay = Delay::new();
    loop {
        stepper.angle(30.0, &delay);
    }
}
