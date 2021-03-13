#![no_main]
#![no_std]

use core::sync::atomic::{self, Ordering};

use cortex_m_rt::entry;
use cortex_m::prelude::*;
use cortex_m_semihosting::hprintln;

use msp432p401r_hal::watchdog::{WatchdogTimer, Enabled, Disable};

pub use panic_abort;

#[entry]
fn main() -> ! {
  // Disable watchdog timer.
  let mut watchdog = WatchdogTimer::<Enabled>::new();
  watchdog.try_disable().unwrap();

  let mut peripherals = cortex_m::Peripherals::take().unwrap();
  let ahb_frequency = 300_0000;
  let mut timer = cortex_m::delay::Delay::new(peripherals.SYST, ahb_frequency);

  let p = msp432p401r::Peripherals::take().unwrap();

  // The Digital I/O module
  let dio = p.DIO;

  hprintln!("Started.");

  // PORTA consists of two ports P1 and P2. Red LED of the
  // MSP432P401R launchpad is on P2.0. Green and Blue LED's
  // on P2.1 and P2.2. Simply set the direction register bit
  // to 1 and write a 1 to the output register to put ON the LED.

  dio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 1) });
  dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 1) });
  dio.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() | 1) });
  dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() & !1) });

  loop {
    hprintln!("Loop.");

    dio.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() ^ 1) });
    dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() ^ 1) });

    timer.delay_ms(1000);
  }
}
