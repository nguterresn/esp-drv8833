use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    ledc::{
        channel::{self, Channel, ChannelIFace},
        timer::{self, config::Duty, Timer, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
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

pub struct MotorTimer<'a> {
    pub timer: Timer<'a, LowSpeed>,
}

impl<'a> MotorTimer<'a> {
    pub fn new(
        ledc: &'a Ledc<'a>,
        timer: timer::Number,
        duty: Duty,
        frequency: Rate,
    ) -> Result<MotorTimer<'a>, Error> {
        let mut lstimer = ledc.timer::<LowSpeed>(timer);
        lstimer.configure(timer::config::Config {
            duty,
            clock_source: timer::LSClockSource::APBClk,
            frequency,
        })?;

        Ok((Self { timer: lstimer }))
    }
}

pub struct MotorLink<T>
where
    T: for<'any> PeripheralOutput<'any>,
{
    channel_num: channel::Number,
    gpio: T,
}

impl<T> MotorLink<T>
where
    T: for<'any> PeripheralOutput<'any>,
{
    pub fn new(channel_num: channel::Number, gpio: T) -> Self {
        Self { channel_num, gpio }
    }
}

pub struct Motor;

impl Motor {
    /// This method links the previously configured timer and GPIOs, A and B.
    ///
    /// It returns a Motor that implements the MotorInterface trait.
    ///
    /// Example:
    ///
    /// ```rust
    /// let motor: MotorFastDecay = Motor::new(&timer, peripherals.GPIO1, peripherals.GPIO2)?;
    /// ```
    pub fn new<'a, M, A, B>(
        ledc: &'a Ledc<'a>,
        timer: &'a Timer<'a, LowSpeed>,
        motor_link_a: MotorLink<A>,
        motor_link_b: MotorLink<B>,
    ) -> Result<M, Error>
    where
        M: MotorInterface<'a>,
        A: for<'any> PeripheralOutput<'any>,
        B: for<'any> PeripheralOutput<'any>,
    {
        let mut channel_a = ledc.channel(motor_link_a.channel_num, motor_link_a.gpio);
        channel_a.configure(channel::config::Config {
            timer: timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })?;

        let mut channel_b = ledc.channel(motor_link_b.channel_num, motor_link_b.gpio);
        channel_b.configure(channel::config::Config {
            timer: timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })?;

        Ok(M::new(channel_a, channel_b))
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
