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
use stm32::{TIM4, RCC};

use hal::prelude::*;
//use hal::digital::v2;

use hal::gpio::{Input, PullDown, Output, PushPull, Alternate, AF2};
use hal::gpio::gpioa;
use hal::gpio::gpiod;

//use hal::delay::Delay;
use hal::hal::Pwm;
//use hal::timer;

enum channel {
    _1,
    _2,
    _3,
    _4,
}

struct Pwm1 {
    period: u32,
    tim: TIM4,
}

impl Pwm1 {
    fn new(timer: TIM4, rcc: &RCC) -> Pwm1 {
        rcc.apb1enr.modify(|_, w| w.tim4en().enabled());
        timer.cr1.modify(|_, w| w.cen().bit(true));
        Pwm1 {
            period: 65535,
            tim: timer,
        }
    }
}

impl Drop for Pwm1 {
    fn drop(&mut self) {
        self.tim.cr1.modify(|_, w| w.cen().bit(false));
    }
}

impl Pwm for Pwm1 {
    type Channel    = channel;
    type Time       = u32;
    type Duty       = f32;

    fn disable(&mut self, channel: Self::Channel) {
        match channel {
            channel::_1 => {
                self.tim.ccmr1_output.modify(|_, w| w.oc1fe().bit(false));
                self.tim.ccmr1_output.modify(|_, w| w.oc1pe().bit(false));
                self.tim.ccmr1_output.modify(|_, w| unsafe {w.oc1m().bits(0b000)});
                self.tim.ccer.modify(|_, w| w.cc1e().bit(false));
            },
            channel::_2 => {
                self.tim.ccmr1_output.modify(|_, w| w.oc2fe().bit(false));
                self.tim.ccmr1_output.modify(|_, w| w.oc2pe().bit(false));
                self.tim.ccmr1_output.modify(|_, w| unsafe {w.oc2m().bits(0b000)});
                self.tim.ccer.modify(|_, w| w.cc2e().bit(false));
            },
            channel::_3 => {
                self.tim.ccmr2_output.modify(|_, w| w.oc3fe().bit(false));
                self.tim.ccmr2_output.modify(|_, w| w.oc3pe().bit(false));
                self.tim.ccmr2_output.modify(|_, w| unsafe {w.oc3m().bits(0b000)});
                self.tim.ccer.modify(|_, w| w.cc3e().bit(false));
            },
            channel::_4 => {
                self.tim.ccmr2_output.modify(|_, w| w.oc4fe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| w.oc4pe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| unsafe {w.oc4m().bits(0b000)});
                self.tim.ccer.modify(|_, w| w.cc4e().bit(false));
            }
        }
    }
    fn enable(&mut self, channel: Self::Channel) {
        self.tim.cr1.modify(|_, w| w.arpe().bit(true));
        self.tim.egr.write(|w| w.ug().bit(true));

        match channel {
            channel::_1 => {
                self.tim.ccmr1_output.modify(|_, w| w.oc1fe().bit(true));
                self.tim.ccmr1_output.modify(|_, w| w.oc1pe().bit(true));
                self.tim.ccmr1_output.modify(|_, w| unsafe {w.oc1m().bits(0b110)});
                self.tim.ccer.modify(|_, w| w.cc1e().bit(true));
            },
            channel::_2 => {
                self.tim.ccmr1_output.modify(|_, w| w.oc2fe().bit(true));
                self.tim.ccmr1_output.modify(|_, w| w.oc2pe().bit(true));
                self.tim.ccmr1_output.modify(|_, w| unsafe {w.oc2m().bits(0b110)});
                self.tim.ccer.modify(|_, w| w.cc2e().bit(true));
            },
            channel::_3 => {
                self.tim.ccmr2_output.modify(|_, w| w.oc3fe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| w.oc3pe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| unsafe {w.oc3m().bits(0b110)});
                self.tim.ccer.modify(|_, w| w.cc3e().bit(true));
            },
            channel::_4 => {
                self.tim.ccmr2_output.modify(|_, w| w.oc4fe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| w.oc4pe().bit(true));
                self.tim.ccmr2_output.modify(|_, w| unsafe {w.oc4m().bits(0b110)});
                self.tim.ccer.modify(|_, w| w.cc4e().bit(true));
            }
        }
    }
    fn get_period(&self) -> Self::Time {
        self.period
    }
    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        match channel {
            channel::_1 => (self.tim.ccr1.read().bits() / self.period) as f32,
            channel::_2 => (self.tim.ccr2.read().bits() / self.period) as f32,
            channel::_3 => (self.tim.ccr3.read().bits() / self.period) as f32,
            channel::_4 => (self.tim.ccr4.read().bits() / self.period) as f32,
        }
    }
    fn get_max_duty(&self) -> Self::Duty {
        1.0
    }
    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        let duty_u32 = (self.period as f32 * duty) as u32;
        match channel {
            channel::_1 => self.tim.ccr1.modify(|_, w| w.ccr().bits(duty_u32)),
            channel::_2 => self.tim.ccr2.modify(|_, w| w.ccr().bits(duty_u32)),
            channel::_3 => self.tim.ccr3.modify(|_, w| w.ccr().bits(duty_u32)),
            channel::_4 => self.tim.ccr4.modify(|_, w| w.ccr().bits(duty_u32)),
        }
    }
    fn set_period<P>(&mut self, period: P) where P: Into<Self::Time> {
        let period_u32 = period.into();
        self.tim.arr.modify(|_, w| w.arr().bits(period_u32));
        self.period = period_u32;
    }
}


#[entry]
fn main() -> ! {
    let _core = stm32::CorePeripherals::take().unwrap();
    let peri = stm32::Peripherals::take().unwrap();

    let mut pwm = Pwm1::new(peri.TIM4, &peri.RCC);

    let gpioa_split = peri.GPIOA.split();
    let gpiod_split = peri.GPIOD.split();
    let pa0: gpioa::PA0<Input<PullDown>> = gpioa_split.pa0.into_pull_down_input();
    let _pd12: gpiod::PD12<Alternate<AF2>> = gpiod_split.pd12.into_alternate_af2();
    let _pd13: gpiod::PD13<Alternate<AF2>> = gpiod_split.pd13.into_alternate_af2();
    let mut pd15: gpiod::PD15<Output<PushPull>> = gpiod_split.pd15.into_push_pull_output();


    //rccregi.apb1enr.modify(|_, w| w.tim4en().enabled());

    let rcc = peri.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    //tim4.egr.modify(|_, w| w.ug().bit(true));

    //tim4.cr1.modify(|_, w| w.arpe().bit(true));


    //tim4.ccmr1_output.modify(|_, w| w.oc1fe().bit(true));
    //tim4.ccmr1_output.modify(|_, w| w.oc1pe().bit(true));
    //tim4.ccmr1_output.modify(|_, w| unsafe {w.oc1m().bits(0b110)});

    //tim4.ccmr1_output.modify(|_, w| w.oc2fe().bit(true));
    //tim4.ccmr1_output.modify(|_, w| w.oc2pe().bit(true));
    //tim4.ccmr1_output.modify(|_, w| unsafe {w.oc2m().bits(0b110)});




    let period: u32 = 0x7FFF;
    pwm.set_period(period);
    pwm.set_duty(channel::_1, 0.5);
    pwm.set_duty(channel::_2, 0.3);

    pwm.enable(channel::_1);
    pwm.enable(channel::_2);

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
