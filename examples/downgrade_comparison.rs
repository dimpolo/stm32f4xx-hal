#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        let p1 = gpioa.pa1.into_push_pull_output().downgrade2();
        let p2 = gpiob.pb2.into_push_pull_output().downgrade2();
        let p3 = gpioc.pc3.into_push_pull_output().downgrade2();
        let p4 = gpioa.pa4.into_push_pull_output().downgrade2();
        let p5 = gpiob.pb5.into_push_pull_output().downgrade2();
        let p6 = gpioc.pc6.into_push_pull_output().downgrade2();

        let pins = [p1,p2,p3,p4,p5,p6];
        toggle(pins);
    }

    loop {}
}

#[inline(never)]
fn toggle<P:embedded_hal::digital::v2::ToggleableOutputPin>(mut pins: [P;6])->!{
    loop {
        for pin in pins.iter_mut(){
            pin.toggle().ok();
        }
    }
}

// cargo size --release --example downgrade_comparison --features stm32f405,bigpin
/*
   text    data     bss     dec     hex filename
   1656       0       4    1660     67c downgrade_comparison
*/

// cargo size --release --example downgrade_comparison --features stm32f405
/*
text    data     bss     dec     hex filename
1640       0       4    1644     66c downgrade_comparison
 */
