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

pub struct MotorConfig<'a> {
    timer: Timer<'a, LowSpeed>,
    ledc: &'a Ledc<'a>,
}

impl<'a> MotorConfig<'a> {
    /// The Motor configuration requires the setting of the _slow clock_ in
    /// the LEDC peripheral.
    ///
    /// ```rust
    /// let mut ledc = Ledc::new(peripherals.LEDC);
    /// ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    /// ```
    ///
    /// Create the motor configuration by setting:
    ///
    /// * The timer, e.g. Timer0, Timer1, etc
    /// * The duty cycle resolution, e.g. 8 bits
    /// * The timer frequency as a Rate type, e.g. Rate::from_khz(20)
    ///
    /// Example:
    ///
    /// ```rust
    /// let motor_conf = MotorConfig::new(
    ///     &ledc,
    ///     timer::Number::Timer0,
    ///     timer::config::Duty::Duty12Bit,
    ///     Rate::from_khz(1),
    /// )?;
    /// ```
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
    /// This method links the motor configuration (MotorConfig) to the passed GPIOs, A and B.
    ///
    /// It returns a Motor that implements the MotorInterface trait.
    ///
    /// Example:
    ///
    /// ```rust
    /// let motor: MotorFastDecay = Motor::new(&motor_conf, peripherals.GPIO1, peripherals.GPIO2)?;
    /// ```
    pub fn new<'a, M, A, B>(motor_config: &'a MotorConfig, gpio_a: A, gpio_b: B) -> Result<M, Error>
    where
        M: MotorInterface<'a>,
        A: PeripheralOutput<'a>,
        B: PeripheralOutput<'a>,
    {
        let mut channel0 = motor_config.ledc.channel(channel::Number::Channel0, gpio_a);
        channel0.configure(channel::config::Config {
            timer: &motor_config.timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })?;

        let mut channel1 = motor_config.ledc.channel(channel::Number::Channel1, gpio_b);
        channel1.configure(channel::config::Config {
            timer: &motor_config.timer,
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
