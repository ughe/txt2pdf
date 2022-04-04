// hello world in Rust
// Version 1 - 41 lines of assembly
// Version 2 (active) - 120 lines of assembly
// Version 3  - 250 lines of assembly

/**********************************************************************
 * VERSION 2                                                          *
 * 120 lines of assembly                                              *
 **********************************************************************/
fn main() {
    print!("hello, world\n");
}

/**********************************************************************
 * VERSION 3                                                          *
 * 250 lines of assembly                                              *
 **********************************************************************/
/*
use std::io::{self, Write};

fn main() -> io::Result<()> {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  handle.write(b"hello, world\n")?;
  Ok(())
}
*/

/**********************************************************************
 * VERSION 1                                                          *
 * 41 lines of assembly                                               *
 * TODO: Uncomment Makefile lines starting with: %_rs: %_rs.o in order*
 *       to compile by linking with glibc for printf. Only works macOS*
 **********************************************************************/
/*
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
        loop {}
}

//extern crate libc;
extern {
  pub fn printf(fmt: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
  unsafe {
    printf(b"hello, world\n" as *const u8);
  }
  0
}
*/
