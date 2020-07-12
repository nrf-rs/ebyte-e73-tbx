#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate ebyte_e73_tbx_bsp;
extern crate nb;
extern crate panic_semihosting;

use core::fmt::Write;
use ebyte_e73_tbx_bsp::{hal::Timer, prelude::*, Board};
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    writeln!(board.cdc_uart, "Hello, world!").unwrap();

    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.led_1.disable();
        } else {
            board.leds.led_1.enable();
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}
