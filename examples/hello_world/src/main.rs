#![feature(no_core)]
#![feature(lang_items)]
#![feature(intrinsics)]
#![feature(naked_functions)]

#![no_core]
#![no_main]

//extern crate core;
use core::alloc::{AllocError, Layout};
use core::arch::asm;
use core::ptr::NonNull;
use core::result::Result::{self, Ok, Err};

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
     asm!(
        "mov rdi, [rsp]",   // argc
        "mov rax, rsp",
        "add rax, 8",
        "mov rsi, rax",     // argv
        "call _start_main",
        options(noreturn),
    )
}

#[no_mangle]
extern "C" fn _start_main(_argc: usize, _argv: *const *const u8) -> ! {
    main();
    exit(40 + 2)
}

#[no_mangle]
fn main() {
    print("Hello world!\n");
}

fn print(string: &str) {
    unsafe {
        write(1, string.as_ptr(), string.len())
    };
}

extern "rust-intrinsic" {
    pub fn abort() -> !;
    pub fn unreachable() -> !;
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
unsafe fn syscall2(n: i64, a1: i64, a2: i64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") n,  // syscall number
        in("rdi") a1, // arg 1
        in("rsi") a2, // arg 2
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

#[inline(always)]
unsafe fn syscall5(n: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") n,  // syscall number
        in("rdi") a1, // arg 1
        in("rsi") a2, // arg 2
        in("rdx") a3, // arg 3
        in("r10") a4, // arg 4
        in("r8")  a5, // arg 5
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
unsafe fn syscall6(n: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") n,  // syscall number
        in("rdi") a1, // arg 1
        in("rsi") a2, // arg 2
        in("rdx") a3, // arg 3
        in("r10") a4, // arg 4
        in("r8")  a5, // arg 5
        in("r9")  a6, // arg 6
        out("rcx") _, // clobbered by syscalls
        out("r11") _, // clobbered by syscalls
        lateout("rax") ret,
    );
    ret
}

#[inline(never)]
#[no_mangle]
fn exit(exit_code: i64) -> ! {
    unsafe {
        syscall1(60, exit_code);
        unreachable()
    }
}

#[inline(never)]
#[no_mangle]
unsafe fn write(fd: i64, buf: *const u8, count: usize) -> i64 {
    syscall3(1, fd, buf as _, count as _)
}

#[inline(never)]
#[no_mangle]
unsafe fn mmap(addr: *const (), length: usize, prot: i32, flags: i32, fd: i32, offset: i32) -> *const () {
    syscall6(9, addr as _, length as _, prot as _, flags as _, fd as _, offset as _) as _
}

#[inline(never)]
#[no_mangle]
unsafe fn mremap(old_address: *const (), old_size: usize, new_size: usize, flags: i32, new_address: *const ()) -> *const () {
    syscall5(25, old_address as _, old_size as _, new_size as _, flags as _, new_address as _) as _
}

#[inline(never)]
#[no_mangle]
unsafe fn munmap(addr: *const (), length: usize) -> i64 {
    syscall2(11, addr as _, length as _)
}

struct MmapAlloc;

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x2;
const MAP_ANONYMOUS: i32 = 0x20;
const MREMAP_MAYMOVE: i32 = 0x1;
// const MREMAP_FIXED: i32 = 0x2;

const MAP_FAILED: *mut u8 = -1 as _;

unsafe impl core::alloc::Allocator for MmapAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.allocate_zeroed(layout)
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe {
            mmap(0 as _, layout.size(), PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) as *mut u8
        };
        if ptr == MAP_FAILED {
            Err(AllocError)
        } else {
            let ptr = unsafe { NonNull::new_unchecked(ptr.cast()) };
            let slice = NonNull::slice_from_raw_parts(ptr, layout.size());
            Ok(slice)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        munmap(ptr.as_ptr().cast(), layout.size());
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_layout.size() == 0 {
            return Err(AllocError)
        }
        if new_layout.size() < old_layout.size() {
            return Err(AllocError)
        }

        let new_ptr = 
            mremap(ptr.as_ptr().cast(), old_layout.size(), new_layout.size(), MREMAP_MAYMOVE, 0 as _) as *mut u8
        ;

        if new_ptr == MAP_FAILED {
            Err(AllocError)
        } else {
            let new_ptr = NonNull::new_unchecked(new_ptr.cast());
            let slice = NonNull::slice_from_raw_parts(new_ptr, new_layout.size());
            Ok(slice)
        }
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_layout.size() > old_layout.size() {
            return Err(AllocError)
        }

        if new_layout.size() == 0 {
            return Err(AllocError)
        }

        let new_ptr = 
            mremap(ptr.as_ptr().cast(), old_layout.size(), new_layout.size(), MREMAP_MAYMOVE, 0 as _) as *mut u8
        ;

        if new_ptr == MAP_FAILED {
            Err(AllocError)
        } else {
            let new_ptr = NonNull::new_unchecked(new_ptr.cast());
            let slice = NonNull::slice_from_raw_parts(new_ptr, new_layout.size());
            Ok(slice)
        }
    }
}
