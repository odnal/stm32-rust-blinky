#![no_std]
#![no_main]
use core::{panic::PanicInfo, ptr::read_volatile, ptr::write_volatile};

extern "C" {
    fn _start();
    fn dummy(_: u32);
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

const RCC_BASE : u32 = 0x40021000;
const RCC_AHB2ENR : *mut u32 = (RCC_BASE + 0x4C) as *mut u32;

const GPIOB_BASE : u32 = 0x48000400;
const GPIOB_MODER : *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_BSRR : *mut u32 = (GPIOB_BASE + 0x18) as *mut u32;

const GPIOE_BASE : u32 = 0x48001000;
const GPIOE_MODER : *mut u32 = (GPIOE_BASE + 0x00) as *mut u32;
const GPIOE_BSRR : *mut u32 = (GPIOE_BASE + 0x18) as *mut u32;

/// GPIO register representation (for those in use for this blinky example)
struct GPIO {
    moder: *mut u32,
    bsrr: *mut u32,
}

/// GPIO operations
pub trait GPIOops {
    fn enable_clock(rcc_ahb2enr: *mut u32, port_reg: u8);
    fn set_mode(&self, pin: u8, mode: u8);
    fn set_pin(&self, pin: u8);
    fn reset_pin(&self, pin: u8);
}

impl GPIOops for GPIO {
    fn enable_clock(rcc_ahb2enr: *mut u32, port_reg: u8) {
        unsafe {
            let ra = read_volatile(rcc_ahb2enr);
            write_volatile(rcc_ahb2enr, ra | (1 << port_reg));
        }
    }

    fn set_mode(&self, pin: u8, mode: u8) {
        unsafe {
            let mut ra = read_volatile(self.moder);
            ra &= !(3 << (pin * 2));
            ra |= (mode as u32) << (pin * 2);
            write_volatile(self.moder, ra);
        }
    }

    fn set_pin(&self, pin: u8) {
        unsafe {
            write_volatile(self.bsrr, 1 << pin);
        }
    }
     
    fn reset_pin(&self, pin: u8) {
        unsafe {
            let offset: u8 = 16;
            write_volatile(self.bsrr, 1 << (pin + offset));
        }
    }
}

#[no_mangle]
pub extern "C" fn notmain() {
    GPIO::enable_clock(RCC_AHB2ENR, 1);
    GPIO::enable_clock(RCC_AHB2ENR, 4);

    let gpio_b = GPIO { moder: GPIOB_MODER, bsrr: GPIOB_BSRR };
    let gpio_e = GPIO { moder: GPIOE_MODER, bsrr: GPIOE_BSRR };

    gpio_b.set_mode(2, 1);
    gpio_e.set_mode(8, 1);

    loop {
        // enable led
        gpio_b.set_pin(2);
        gpio_e.set_pin(8);
        for ra in 0..0x1000 { unsafe { dummy(ra); } }

        // disable led
        gpio_b.reset_pin(2);
        gpio_e.reset_pin(8);
        for ra in 0..0x1000 { unsafe { dummy(ra); } }
    }
}
