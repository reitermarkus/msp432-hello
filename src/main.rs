#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::prelude::*;
use cortex_m_semihosting::hprintln;

use msp432p401r_hal::watchdog::{WatchdogTimer, Enabled, Disable};
use msp432p401r_hal::{gpio::{GpioExt, InputPin, OutputPin, ToggleableOutputPin}};

use panic_abort as _;

#[entry]
fn main() -> ! {
  // Disable watchdog timer.
  let watchdog = WatchdogTimer::<Enabled>::new();
  watchdog.try_disable().unwrap();

  let peripherals = cortex_m::Peripherals::take().unwrap();
  let ahb_frequency = 3_000_000;
  let mut timer = cortex_m::delay::Delay::new(peripherals.SYST, ahb_frequency);

  let p = msp432p401r::Peripherals::take().unwrap();

  // The Digital I/O module
  let dio = p.DIO.split();

  hprintln!("Started.").unwrap();

  // PORTA consists of two ports P1 and P2. Red LED of the
  // MSP432P401R launchpad is on P2.0. Green and Blue LED's
  // on P2.1 and P2.2. Simply set the direction register bit
  // to 1 and write a 1 to the output register to put ON the LED.

  let mut led1 = dio.p1_0.into_output();
  led1.try_set_low().unwrap();

  let mut rgbled_red = dio.p2_0.into_output();
  rgbled_red.try_set_high().unwrap();
  let mut rgbled_green = dio.p2_1.into_output();
  rgbled_green.try_set_high().unwrap();
  let mut rgbled_blue = dio.p2_2.into_output();
  rgbled_blue.try_set_high().unwrap();

  let button1 = dio.p1_4.into_pull_up_input();
  let button2 = dio.p1_1.into_pull_up_input();

  loop {
    hprintln!("Loop.").unwrap();

    led1.try_toggle().unwrap();
    rgbled_red.try_toggle().unwrap();
    rgbled_green.try_toggle().unwrap();
    rgbled_blue.try_toggle().unwrap();

    hprintln!("Button 1: {}", button1.try_is_low().unwrap()).unwrap();
    hprintln!("Button 2: {}", button2.try_is_low().unwrap()).unwrap();

    timer.delay_ms(1000);
  }
}
