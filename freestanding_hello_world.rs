#![feature(no_core)]
#![feature(lang_items)]
#![feature(intrinsics)]
#![feature(naked_functions)]
#![feature(decl_macro)]
#![feature(rustc_attrs)]
#![no_core]
#![no_main]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}

extern "rust-intrinsic" {
    pub fn abort() -> !;
    pub fn unreachable() -> !;
}

impl Copy for i64 {}

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    asm!(
        "mov rdi, [rsp]", // argc
        "mov rax, rsp",
        "add rax, 8",
        "mov rsi, rax", // argv
        "call _start_main",
        options(noreturn),
    )
}

#[no_mangle]
extern "C" fn _start_main(_argc: usize, _argv: *const *const u8) -> ! {
    main();
    exit(0)
}

fn main() {
    let string = b"Hello World!\n" as *const _ as *const u8;
    write(1, string, 13);
}

#[inline(never)]
#[no_mangle]
fn write(fd: i64, data: *const u8, len: i64) -> i64 {
    unsafe { syscall3(1, fd, data as i64, len) }
}

#[inline(never)]
#[no_mangle]
fn exit(exit_code: i64) -> ! {
    unsafe {
        syscall1(60, exit_code);
        unreachable()
    }
}

#[inline(always)]
unsafe fn syscall1(n: i64, a1: i64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") n,  // syscall number
        in("rdi") a1, // arg 1
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
unsafe fn syscall3(n: i64, a1: i64, a2: i64, a3: i64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") n,  // syscall number
        in("rdi") a1, // arg 1
        in("rsi") a2, // arg 2
        in("rdx") a3, // arg 3
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}

