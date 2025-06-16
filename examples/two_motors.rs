#![no_std]
#![no_main]

use esp_drv8833::drv8833::MotorLink;
use esp_drv8833::{Motor, MotorFastDecay, MotorInterface, MotorTimer};
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

    let motor_timer_conf = MotorTimer::new(
        &ledc,
        timer::Number::Timer0,
        timer::config::Duty::Duty12Bit,
        Rate::from_khz(1),
    )
    .unwrap();

    let motor_right: MotorFastDecay = Motor::new(
        &ledc,
        &motor_timer_conf.timer,
        MotorLink::new(channel::Number::Channel0, peripherals.GPIO1),
        MotorLink::new(channel::Number::Channel1, peripherals.GPIO2),
    )
    .unwrap();
    let motor_left: MotorFastDecay = Motor::new(
        &ledc,
        &motor_timer_conf.timer,
        MotorLink::new(channel::Number::Channel2, peripherals.GPIO3),
        MotorLink::new(channel::Number::Channel3, peripherals.GPIO4),
    )
    .unwrap();

    motor_right.forward(100).unwrap();
    motor_left.forward(50).unwrap();

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
