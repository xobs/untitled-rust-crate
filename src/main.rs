#![no_main]
#![no_std]

use core::panic::PanicInfo;
use riscv_rt::entry;
use betrusted_pac;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}


#[entry]
fn main() -> ! {
    let mut cnt = 64;
    let peripherals = betrusted_pac::Peripherals::take().unwrap();
    loop {
        while peripherals.UART.txfull.read().txfull() == true {
        }
        peripherals.UART.rxtx.write(|x| unsafe { x.rxtx().bits(cnt) });
        cnt = match cnt {
            x if x < 120 => x + 1,
            _ => 64,
        };
    }
}