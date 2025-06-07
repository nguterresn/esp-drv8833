# About

This crate provides control over the DRV8833 Dual H-Bridge Motor Driver.

The crate requires no external or standard library and only depends on the [esp-hal](https://github.com/esp-rs/esp-hal) crate.
The code uses the espressif [LEDC](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/ledc.html) peripheral to control the DRV8833.

The crate uses the [slow clock](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.1/esp32c6/esp_hal/ledc/enum.LSGlobalClkSource.html) as default:

* It is better suited for motor control, since the frenquency is quite low, e.g. < 20kHz.
* It is more power efficient.
* It can still work under sleep modes.

### Drive forward with 100% duty cycle

The followig example shows how to use the crate to drive a brushed motor
forward with max duty cycle (100%), using the GPIO1 and GPIO2:

```rust
let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
let peripherals = esp_hal::init(config);

let mut ledc = Ledc::new(peripherals.LEDC);
ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

let motor_conf = MotorTimerConfig::new(
    &ledc,
    timer::Number::Timer0,
    timer::config::Duty::Duty12Bit,
    Rate::from_khz(1),
)?;

let motor: MotorFastDecay = Motor::new(&motor_conf, peripherals.GPIO1, peripherals.GPIO2)?;

motor.forward(100)?;
```

### Drive backwards with 50% duty cycle

```rust
motor.backward(50)?;
```

### Brake motor

```rust
motor.brake()?;
```

### Setup a slow decay motor

```rust
let motor: MotorSlowDecay = Motor::new(&motor_conf, peripherals.GPIO1, peripherals.GPIO2)?;
```

# Build

```BASH
cargo build --example forward --features esp32c6
```
