#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    unsafe {
            core::arch::asm!(
            "addi sp, sp, -4",
            "sw a0, (sp)",
            "li a7, 93",
            "ecall",
            options(noreturn)
        )
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
