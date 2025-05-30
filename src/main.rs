#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(photon::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  println!("{}", "Debug");

  photon::init();

  #[cfg(test)]
  test_main();

  #[allow(clippy::empty_loop)]
  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  photon::test_panic_handler(info)
}

#[allow(clippy::eq_op)]
#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}
