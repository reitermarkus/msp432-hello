#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::prelude::*;
use panic_abort as _;
use cortex_m_semihosting::hprintln;
use micromath::F32Ext;

use msp432p401r::{interrupt, TIMER_A0, TIMER_A1};
use msp432p401r_hal::{
  clock::{CsExt, DcoclkFreqSel, DIVM_A, DIVS_A},
  timer::*,
  watchdog::{WatchdogTimer, Disable},
  pmap::Mapping,
};

use msp432p401r_hal::gpio::GpioExt;
use msp432p401r_hal::gpio::{OutputPin, ToggleableOutputPin};
use msp432p401r_hal::gpio::Output;
use msp432p401r_hal::{flash::{FlashConfig, FlcDefined, FlWaitSts}};

use lazy_static::lazy_static;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

lazy_static! {
  static ref LED: Mutex<RefCell<Option<msp432p401r_hal::gpio::porta::P1_0<Output>>>> = Mutex::new(RefCell::new(None));
  static ref TIMER_1: Mutex<RefCell<Option<UpTimer<TIMER_A0>>>> = Mutex::new(RefCell::new(None));
  static ref TIMER_2: Mutex<RefCell<Option<UpTimer<TIMER_A1>>>> = Mutex::new(RefCell::new(None));
}

const TA0_CCR0_BREATH: u16 = 999;
const PWM_CCR0: u16 = 255;

#[entry]
fn main() -> ! {
  let mut p = msp432p401r::Peripherals::take().unwrap();

  let watchdog = WatchdogTimer::new(p.WDT_A);
  watchdog.try_disable().unwrap();

  let dio = p.DIO.split();

  let mut led1 = dio.p1_0.into_output();
  led1.try_set_high().unwrap();
  let mut led2 = dio.p2_0.into_output().into_alternate_primary();
  led2.remap(Mapping::TA1CCR1A, &mut p.PMAP);
  let mut led3 = dio.p2_1.into_output().into_alternate_primary();
  led3.remap(Mapping::TA1CCR1A, &mut p.PMAP);
  let mut led4 = dio.p2_2.into_output().into_alternate_primary();
  led4.remap(Mapping::TA1CCR1A, &mut p.PMAP);

  let mut timer1 = Timer::a0(p.TIMER_A0,/* &clocks*/);
  let mut timer2 = Timer::a1(p.TIMER_A1,/* &clocks*/);

  cortex_m::interrupt::free(|cs| {
    LED.borrow(cs).replace(Some(led1));
  });

  const period: u16 = (3_000_000 as u32 / 64) as u16;

  let up_mode_config1 = UpTimerConfig {
    clock_source: ClockSource::SMCLK,
    divider: ClockSourceDivider::_64,
    period: TA0_CCR0_BREATH,
    interrupt_enabled: false,
    capture_compare_interrupt_enabled: true,
    clear: true,
  };

  let mut timer1 = timer1.up(&up_mode_config1);

  cortex_m::interrupt::free(move |cs| {
    timer1.clear_capture_compare_interrupt(0);
    timer1.start();
    TIMER_1.borrow(cs).replace(Some(timer1));
  });

  unsafe {
    cortex_m::peripheral::NVIC::unmask(msp432p401r::Interrupt::TA0_0_IRQ);
    cortex_m::interrupt::enable();
  }

  let up_mode_config2 = UpTimerConfig {
    clock_source: ClockSource::SMCLK,
    divider: ClockSourceDivider::_1,
    period: PWM_CCR0,
    interrupt_enabled: false,
    capture_compare_interrupt_enabled: false,
    clear: true,
  };

  let compare_config2 = CompareConfig {
    capture_compare_interrupt_enabled: false,
    output_mode: OutputMode::ToggleReset,
    compare_value: PWM_CCR0 - 1,
  };

  let mut timer2 = timer2.up(&up_mode_config2);

  cortex_m::interrupt::free(move |cs| {
    timer2.set_compare_config(1, &compare_config2);
    timer2.start();
    TIMER_2.borrow(cs).replace(Some(timer2));
  });

  loop {}
}

#[interrupt]
fn TA0_0_IRQ() {
  static mut duty_cycle: u16 = 0;

  use core::f32::consts::PI;

  *duty_cycle += 1;
  if (*duty_cycle > PWM_CCR0) {
    *duty_cycle -= PWM_CCR0;
  }

  cortex_m::interrupt::free(|cs| {
    let mut led = LED.borrow(cs).borrow_mut();
    led.as_mut().unwrap().try_toggle().unwrap();

    let mut timer1 = TIMER_1.borrow(cs).borrow_mut();
    timer1.as_mut().unwrap().clear_capture_compare_interrupt(0);

    let mut timer2 = TIMER_2.borrow(cs).borrow_mut();
    let f: f32 = PI * *duty_cycle as f32 / PWM_CCR0 as f32;
    timer2.as_mut().unwrap().set_compare_value(1, (f.sin() * PWM_CCR0 as f32) as u16);
  })
}
