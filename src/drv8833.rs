#![allow(dead_code)]

use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    ledc::{
        channel::{self, Channel, ChannelIFace},
        timer::{self, config::Duty, Number, Timer, TimerIFace},
        Ledc, LowSpeed,
    },
    time::Rate,
};

#[derive(Debug)]
pub enum Error {
    ChannelError(esp_hal::ledc::channel::Error),
    TimerError(esp_hal::ledc::timer::Error),
}

impl From<esp_hal::ledc::channel::Error> for Error {
    fn from(error: esp_hal::ledc::channel::Error) -> Self {
        Error::ChannelError(error)
    }
}

impl From<esp_hal::ledc::timer::Error> for Error {
    fn from(error: esp_hal::ledc::timer::Error) -> Self {
        Error::TimerError(error)
    }
}

#[derive(PartialEq)]
pub enum MotorDecay {
    FastDecay,
    SlowDecay,
}

pub struct MotorConfig<'a> {
    timer: Timer<'a, LowSpeed>,
    ledc: &'a Ledc<'a>,
}

impl<'a> MotorConfig<'a> {
    pub fn new(
        ledc: &'a Ledc<'a>,
        timer: Number,
        duty: Duty,
        frequency: Rate,
    ) -> Result<Self, Error> {
        let mut lstimer = ledc.timer::<LowSpeed>(timer);
        lstimer.configure(timer::config::Config {
            duty,
            clock_source: timer::LSClockSource::APBClk,
            frequency,
        })?;

        Ok(Self {
            ledc: ledc,
            timer: lstimer,
        })
    }
}

pub struct Motor;

impl Motor {
    pub fn new<'a, M, A, B>(driver: &'a Driver, gpio_a: A, gpio_b: B) -> Result<M, Error>
    where
        M: MotorInterface<'a>,
        A: PeripheralOutput<'a>,
        B: PeripheralOutput<'a>,
    {
        let mut channel0 = driver.ledc.channel(channel::Number::Channel0, gpio_a);
        channel0.configure(channel::config::Config {
            timer: &driver.timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })?;

        let mut channel1 = driver.ledc.channel(channel::Number::Channel1, gpio_b);
        channel1.configure(channel::config::Config {
            timer: &driver.timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })?;

        Ok(M::new(channel0, channel1))
    }
}

pub trait MotorInterface<'a> {
    fn new(a: Channel<'a, LowSpeed>, b: Channel<'a, LowSpeed>) -> Self;
    fn forward(&self, duty: u8) -> Result<(), Error>;
    fn backward(&self, duty: u8) -> Result<(), Error>;
    fn brake(&self) -> Result<(), Error>;
}

pub struct MotorFastDecay<'a> {
    a: Channel<'a, LowSpeed>,
    b: Channel<'a, LowSpeed>,
}

impl<'a> MotorInterface<'a> for MotorFastDecay<'a> {
    fn new(a: Channel<'a, LowSpeed>, b: Channel<'a, LowSpeed>) -> Self {
        Self { a, b }
    }

    fn forward(&self, duty: u8) -> Result<(), Error> {
        self.a.set_duty(duty)?;
        self.b.set_duty(0)?;

        Ok(())
    }

    fn backward(&self, duty: u8) -> Result<(), Error> {
        self.a.set_duty(0)?;
        self.b.set_duty(duty)?;

        Ok(())
    }

    fn brake(&self) -> Result<(), Error> {
        self.a.set_duty(0)?;
        self.b.set_duty(0)?;

        Ok(())
    }
}

pub struct MotorSlowDecay<'a> {
    a: Channel<'a, LowSpeed>,
    b: Channel<'a, LowSpeed>,
}

impl<'a> MotorInterface<'a> for MotorSlowDecay<'a> {
    fn new(a: Channel<'a, LowSpeed>, b: Channel<'a, LowSpeed>) -> Self {
        Self { a, b }
    }

    fn forward(&self, duty: u8) -> Result<(), Error> {
        self.a.set_duty(100)?;
        self.b.set_duty(100 - duty)?;

        Ok(())
    }

    fn backward(&self, duty: u8) -> Result<(), Error> {
        self.a.set_duty(100 - duty)?;
        self.b.set_duty(100)?;

        Ok(())
    }

    fn brake(&self) -> Result<(), Error> {
        self.a.set_duty(100)?;
        self.b.set_duty(100)?;

        Ok(())
    }
}
