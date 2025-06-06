#![no_std]

pub mod drv8833;

pub use drv8833::Motor;
pub use drv8833::MotorConfig;
pub use drv8833::MotorFastDecay;
pub use drv8833::MotorInterface;
pub use drv8833::MotorSlowDecay;
