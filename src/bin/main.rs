#![no_std]
#![no_main]

use esp_drv8833::{Motor, MotorConfig, MotorFastDecay, MotorInterface};
use esp_hal::clock::CpuClock;
use esp_hal::ledc::{timer, LSGlobalClkSource, Ledc};
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use esp_println::println;

#[derive(Debug)]
pub enum Error {
    Other,
    MCPWMError(esp_hal::mcpwm::FrequencyError),
    LEDCTimerError(esp_hal::ledc::timer::Error),
    LEDCChannelError(esp_hal::ledc::channel::Error),
}

impl From<esp_hal::mcpwm::FrequencyError> for Error {
    fn from(error: esp_hal::mcpwm::FrequencyError) -> Self {
        Error::MCPWMError(error)
    }
}

impl From<esp_hal::ledc::timer::Error> for Error {
    fn from(error: esp_hal::ledc::timer::Error) -> Self {
        Error::LEDCTimerError(error)
    }
}

impl From<esp_hal::ledc::channel::Error> for Error {
    fn from(error: esp_hal::ledc::channel::Error) -> Self {
        Error::LEDCChannelError(error)
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!(
        "Panic! Mesage: {} location: {:?}",
        info.message(),
        info.location()
    );
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let motor_conf = MotorConfig::new(
        &ledc,
        timer::Number::Timer0,
        timer::config::Duty::Duty12Bit,
        Rate::from_khz(1),
    )
    .expect("Failed to setup DRV8833");

    let motor: MotorFastDecay = Motor::new(&motor_conf, peripherals.GPIO1, peripherals.GPIO2)
        .expect("Failed to create motor right");

    motor.forward(100).expect("Failed to set duty cycle to 50%");

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
