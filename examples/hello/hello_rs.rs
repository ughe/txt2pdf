/*
 * VERSION 1
 */
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

#[no_mangle]
pub extern "C" fn __start() -> i32 {
  return main(0, 0 as *const *const u8);
}

/*
 * VERSION 2
 */
/*
fn main() { // 120 lines ASM
    print!("hello, world\n");
}
*/

/*
 * VERSION 3
 */
/*
use std::io::{self, Write};

fn main() -> io::Result<()> { // 250 lines ASM
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  handle.write(b"hello, world\n")?;
  Ok(())
}
*/
