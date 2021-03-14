#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::prelude::*;
use cortex_m_semihosting::hprintln;

use msp432p401r::CS;
use msp432p401r_hal::watchdog::{WatchdogTimer, Enabled, Disable};
use msp432p401r_hal::{clock::{CsExt, DcoclkFreqSel, DIVM_A, DIVS_A}, gpio::{GpioExt, InputPin, OutputPin, ToggleableOutputPin}, pcm::{PcmConfig, PcmDefined, VCoreSel}};

use panic_abort as _;

#[entry]
fn main() -> ! {
  // Disable watchdog timer.
  let watchdog = WatchdogTimer::<Enabled>::new();
  watchdog.try_disable().unwrap();

  let mut pcm: PcmConfig::<PcmDefined> = PcmConfig::<PcmDefined>::new();   // Setup PcmConfig
          pcm.set_vcore(VCoreSel::DcdcVcore1);                             // Set DCDC Vcore1 -> 48 MHz Clock
  let _pcm_sel = pcm.get_powermode();                                      // Get the current powermode

  // hprintln!("Power Mode: {:?}", pcm_sel).unwrap();


  let p = msp432p401r::Peripherals::take().unwrap();

  // The Digital I/O module
  let dio = p.DIO.split();

  let freq_sel = DcoclkFreqSel::_24MHz;

  let _clocks = p.CS.constrain()
      .mclk_dcoclk(freq_sel, DIVM_A::DIVM_0)
      .smclk_div(DIVS_A::DIVS_1)
      .freeze();

  let peripherals = cortex_m::Peripherals::take().unwrap();
  let ahb_frequency = freq_sel.freq();
  let mut timer = cortex_m::delay::Delay::new(peripherals.SYST, ahb_frequency);

  #[cfg(debug)]
  hprintln!("Started.").unwrap();

  // PORTA consists of two ports P1 and P2. Red LED of the
  // MSP432P401R launchpad is on P2.0. Green and Blue LED's
  // on P2.1 and P2.2. Simply set the direction register bit
  // to 1 and write a 1 to the output register to put ON the LED.

  let mut led1 = dio.p1_0.into_output();
  led1.try_set_high().unwrap();

  let mut rgbled_red = dio.p2_0.into_output();
  rgbled_red.try_set_low().unwrap();
  let mut rgbled_green = dio.p2_1.into_output();
  rgbled_green.try_set_low().unwrap();
  let mut rgbled_blue = dio.p2_2.into_output();
  rgbled_blue.try_set_low().unwrap();

  let button1 = dio.p1_4.into_pull_up_input();
  let button2 = dio.p1_1.into_pull_up_input();

  let mut color = 0;

  loop {
    // hprintln!("Loop.").unwrap();

    led1.try_toggle().unwrap();

    match color {
      0..=1 => {
        rgbled_red.try_toggle().unwrap();
      }
      2..=3 => {
        rgbled_green.try_toggle().unwrap();
      }
      4..=5 => {
        rgbled_blue.try_toggle().unwrap();
      }
      _ => unreachable!() ,
    }

    color = (color + 1) % 6;


    // hprintln!("Button 1: {}", button1.try_is_low().unwrap()).unwrap();
    // hprintln!("Button 2: {}", button2.try_is_low().unwrap()).unwrap();

    timer.delay_ms(100);
  }
}
