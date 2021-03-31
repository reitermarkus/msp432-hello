#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::prelude::*;
use cortex_m_semihosting::hprintln;

use msp432p401r::interrupt;
use msp432p401r_hal::watchdog::{WatchdogTimer, Disable};
use msp432p401r_hal::{flash::{FlashConfig, FlcDefined, FlWaitSts}, clock::{CsExt, DcoclkFreqSel, DIVM_A, DIVS_A}, gpio::{Edge, Input, PullUp, GpioExt, OutputPin, StatefulOutputPin, ToggleableOutputPin}, pcm::{PcmConfig, PcmDefined, VCoreSel}};

use panic_abort as _;

use lazy_static::lazy_static;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;


lazy_static! {
  static ref MUTEX_BUTTON1: Mutex<RefCell<Option<msp432p401r_hal::gpio::porta::P1_4<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
  static ref MUTEX_BUTTON2: Mutex<RefCell<Option<msp432p401r_hal::gpio::porta::P1_1<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
  static ref COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
}

#[interrupt]
fn PORT1_IRQ() {
  cortex_m::interrupt::free(|cs| {
    let mut counter = COUNTER.borrow(cs).borrow_mut();

    let mut button1 = MUTEX_BUTTON1.borrow(cs).borrow_mut();
    if button1.as_mut().unwrap().check_interrupt() {
      button1.as_mut().unwrap().clear_interrupt_pending_bit();
      *counter = (*counter + 1) % 7;
    }

    let mut button2 = MUTEX_BUTTON2.borrow(cs).borrow_mut();
    if button2.as_mut().unwrap().check_interrupt() {
      button2.as_mut().unwrap().clear_interrupt_pending_bit();
      *counter = (*counter + 7 - 1) % 7;
    }

  });
}

#[entry]
fn main() -> ! {
  let p = msp432p401r::Peripherals::take().unwrap();

  // Disable watchdog timer.
  let watchdog = WatchdogTimer::new(p.WDT_A);
  watchdog.try_disable().unwrap();

  let mut pcm: PcmConfig::<PcmDefined> = PcmConfig::<PcmDefined>::new();   // Setup PcmConfig
          pcm.set_vcore(VCoreSel::DcdcVcore1);                             // Set DCDC Vcore1 -> 48 MHz Clock
  let _pcm_sel = pcm.get_powermode();                                      // Get the current powermode

  // hprintln!("Power Mode: {:?}", pcm_sel).unwrap();

  let flctl = FlashConfig::<FlcDefined>::new();         // Setup FlashConfig
  flctl.set_flwaitst(FlWaitSts::_2Ws);                                       // Two wait states -> 48 Mhz Clock


  // The Digital I/O module
  let dio = p.DIO.split();

  let freq_sel = DcoclkFreqSel::_48MHz;

  let _clocks = p.CS.constrain()
      .mclk_dcoclk(freq_sel, DIVM_A::DIVM_0)
      .smclk_div(DIVS_A::DIVS_1)
      .freeze();

  let peripherals = cortex_m::Peripherals::take().unwrap();
  let ahb_frequency = freq_sel.freq();
  let mut timer = cortex_m::delay::Delay::new(peripherals.SYST, ahb_frequency);

  #[cfg(debug_assertions)]
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

  let mut button1 = dio.p1_4.into_pull_up_input();
  let mut button2 = dio.p1_1.into_pull_up_input();

  button1.trigger_on_edge(Edge::Rising);
  button2.trigger_on_edge(Edge::Rising);

  button1.enable_interrupt();
  button2.enable_interrupt();

  button1.trigger_on_edge(Edge::Falling);
  button2.trigger_on_edge(Edge::Falling);

  cortex_m::interrupt::free(|cs| {
    MUTEX_BUTTON1.borrow(cs).replace(Some(button1));
    MUTEX_BUTTON2.borrow(cs).replace(Some(button2));
  });

  unsafe {
    cortex_m::peripheral::NVIC::unmask(msp432p401r::Interrupt::PORT1_IRQ);
  }

  loop {
    // hprintln!("Loop.").unwrap();

    led1.try_toggle().unwrap();

    let color = cortex_m::interrupt::free(|cs| {
      *COUNTER.borrow(cs).borrow() + 1
    });

    if led1.try_is_set_low().unwrap() && color & 0b100 != 0 {
      rgbled_red.try_set_high().unwrap();
    } else {
      rgbled_red.try_set_low().unwrap();
    }

    if led1.try_is_set_low().unwrap() && color & 0b010 != 0 {
      rgbled_green.try_set_high().unwrap();
    } else {
      rgbled_green.try_set_low().unwrap();
    }

    if led1.try_is_set_low().unwrap() && color & 0b001 != 0 {
      rgbled_blue.try_set_high().unwrap();
    } else {
      rgbled_blue.try_set_low().unwrap();
    }

    // color = (color + 1) % 6;

    // hprintln!("Button 1: {}", button1.interrupt_enabled()).unwrap();
    // hprintln!("Button 2: {}", button2.interrupt_enabled()).unwrap();
    //
    //
    // hprintln!("Button 1: {}", button1.try_is_low().unwrap()).unwrap();
    // hprintln!("Button 2: {}", button2.try_is_low().unwrap()).unwrap();

    timer.delay_ms(250);
  }
}
