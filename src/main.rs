#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cipherdogs_os_art::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cipherdogs_os_art::logo;
use cipherdogs_os_art::println;
use cipherdogs_os_art::vga;
use cipherdogs_os_art::window;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().clear_all();

    window::create_center();
    window::write_string(10, 25, "CipherDogs");
    window::write_string(14, 22, "Cyber-crypto team");
    logo::draw(7, 40, 17, 58);

    println!(
        "Copyleft {} CipherDogs | Source code available under the GPL-3.0",
        "2020"
    );

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
