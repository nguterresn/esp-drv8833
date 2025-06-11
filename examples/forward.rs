#![no_std]
#![no_main]

use esp_drv8833::drv8833::MotorLink;
use esp_drv8833::{Motor, MotorFastDecay, MotorInterface, MotorTimerConfig};
use esp_hal::clock::CpuClock;
use esp_hal::ledc::{channel, timer, LSGlobalClkSource, Ledc};
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

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let motor_timer_conf = MotorTimerConfig::new(
        &ledc,
        timer::Number::Timer0,
        timer::config::Duty::Duty12Bit,
        Rate::from_khz(1),
    )
    .expect("Failed to setup DRV8833");

    // A channel number from 0-7;
    let motor: MotorFastDecay = Motor::new(
        &ledc,
        &motor_timer_conf,
        MotorLink::new(channel::Number::Channel0, peripherals.GPIO1),
        MotorLink::new(channel::Number::Channel1, peripherals.GPIO2),
    )
    .expect("Failed to create motor right");

    motor.forward(100).expect("Failed to set duty cycle to 50%");

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
