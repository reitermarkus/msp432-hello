#![no_main]
#![no_std]

use cortex_m_rt::entry;

pub use panic_abort;

#[entry]
fn main() -> ! {
  let p = msp432p401r::Peripherals::take().unwrap();

  // The Digital I/O module
  let dio = p.DIO;

  // PORTA consists of two ports P1 and P2. Red LED of the
  // MSP432P401R launchpad is on P2.0. Green and Blue LED's
  // on P2.1 and P2.2. Simply set the direction register bit
  // to 1 and write a 1 to the output register to put ON the LED.
  dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 1) });

  dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() | 1) });

  loop {}
}
