#![no_std]
#![feature(naked_functions)]

core::arch::global_asm!(
    r"
.macro LDR rd, rs, off
    ld \rd, \off*8(\rs)
.endm
",
);

#[naked]
/// # Safety
pub unsafe extern "C" fn context_switch() {
    unsafe {
        core::arch::naked_asm!(
            "
        // omit code around
        LDR     s11, a1, 13

        ret",
        )
    }
}
