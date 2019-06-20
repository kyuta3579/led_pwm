#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f4xx_hal as hal;

//use cortex_m::asm;
use cortex_m_rt::entry;
use core::ops::Deref;
use hal::stm32;

use hal::prelude::*;
//use hal::digital::v2;

use hal::gpio::{Input, PullDown, Output, PushPull, Alternate, AF2};
use hal::gpio::gpioa;
use hal::gpio::gpiod;
//use hal::gpio::Speed;

//use hal::delay::Delay;
//use hal::hal::Pwm;
//use hal::timer;

#[entry]
fn main() -> ! {
    let _core = stm32::CorePeripherals::take().unwrap();
    let peri = stm32::Peripherals::take().unwrap();

    let rccregi = peri.RCC.deref();
    let tim4 = peri.TIM4.deref();

    let gpioa_split = peri.GPIOA.split();
    let gpiod_split = peri.GPIOD.split();
    let pa0: gpioa::PA0<Input<PullDown>> = gpioa_split.pa0.into_pull_down_input();
    let _pd12: gpiod::PD12<Alternate<AF2>> = gpiod_split.pd12.into_alternate_af2();
    let _pd13: gpiod::PD13<Alternate<AF2>> = gpiod_split.pd13.into_alternate_af2();
    let mut pd15: gpiod::PD15<Output<PushPull>> = gpiod_split.pd15.into_push_pull_output();


    rccregi.apb1enr.modify(|_, w| w.tim4en().enabled());

    let rcc = peri.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    tim4.egr.write(|w| w.ug().bit(true));

    tim4.cr1.modify(|_, w| w.arpe().bit(true));

    //tim4.ccmr1_output.modify(|_, w| unsafe {w.cc1s().bits(0b00)});
    tim4.ccmr1_output.modify(|_, w| w.oc1fe().bit(true));
    tim4.ccmr1_output.modify(|_, w| w.oc1pe().bit(true));
    tim4.ccmr1_output.modify(|_, w| unsafe {w.oc1m().bits(0b110)});
    //tim4.ccmr1_output.write(|w| w.oc1ce().bit(false));
    //tim4.ccmr1_output.write(|w| unsafe {w.cc2s().bits(0b00)});
    tim4.ccmr1_output.modify(|_, w| w.oc2fe().bit(true));
    tim4.ccmr1_output.modify(|_, w| w.oc2pe().bit(true));
    tim4.ccmr1_output.modify(|_, w| unsafe {w.oc2m().bits(0b110)});
    //tim4.ccmr1_output.modify(|_, w| w.oc2ce().bit(false));

    tim4.arr.modify(|_, w| w.arr().bits(0x7FFF));
    tim4.ccr1.modify(|_, w| w.ccr().bits(0x3FFF));
    tim4.ccr2.modify(|_, w| w.ccr().bits(0x5FFF));

    tim4.ccer.modify(|_, w| w.cc1e().bit(true));
    tim4.ccer.modify(|_, w| w.cc2e().bit(true));

    tim4.cr1.modify(|_, w| w.cen().bit(true));

    

    //let mut delay = Delay::new(core.SYST, clocks);
    
    loop {
        // your code goes here
        if pa0.is_low() {
            pd15.set_high();
        }
        if pa0.is_high(){
            pd15.set_low();
        }
    }
}
