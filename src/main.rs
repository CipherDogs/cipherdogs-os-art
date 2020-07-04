#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cipherdogs_os_art::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cipherdogs_os_art::println;
use cipherdogs_os_art::vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //vga::WRITER.lock().clear_all();
    let fg = vga::Color::Black;
    let bg = vga::Color::White;

    vga::WRITER.lock().write_xy_color_byte(2, 1, fg, bg, b' ');
    vga::WRITER.lock().write_xy_color_byte(3, 1, fg, bg, b' ');
    vga::WRITER.lock().write_xy_color_byte(4, 1, fg, bg, b' ');

    vga::WRITER.lock().write_xy_byte(6, 5, b't');
    println!("Copyleft {} CipherDogs | Source code available under the AGPL", "2020");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cipherdogs_os_art::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}