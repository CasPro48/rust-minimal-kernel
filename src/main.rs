//! Rust Minimal Kernel
//! A minimalistic OS kernel written entirely in Rust

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

/// Kernel entry point called by the bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Clear screen first
    vga_buffer::WRITER.lock().clear_screen();
    
    println!("========================================");
    println!("  Rust Minimal Kernel v0.1.0");
    println!("  A minimalistic OS written in Rust");
    println!("========================================");
    println!();
    println!("Kernel initialized successfully!");
    println!("VGA text mode: 80x25 characters");
    println!();
    println!("System ready. Welcome to your Rust OS!");

    #[cfg(test)]
    test_main();

    loop {
        // Halt CPU to save power while idle
        x86_64::instructions::hlt();
    }
}

/// Panic handler - called on panic in no_std environment
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n[KERNEL PANIC]");
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
