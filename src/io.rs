use core::ffi::c_void;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

const IO_ERROR_NO_ENTRY: u32 = 0x80010002;

static mut DATA: [u8; 16] = [0; 16];

extern "C" fn tile_read_ready(a: i32, b: i32, arg: *mut c_void) -> i32 {
    psp::dprintln!("a: {a}, b: {b}, arg: {arg:?}");
    psp::dprintln!("data: {:?}", unsafe { DATA });
    0
}

pub fn start_reading_tile(path: &str) {
    unsafe {
        let fd = psp::sys::sceIoOpen(path.as_ptr(), psp::sys::IoOpenFlags::RD_ONLY, 0);
        if fd.0 < 0 {
            todo!("Return error here");
        }

        let cb = psp::sys::sceKernelCreateCallback(
            "tile_read_ready".as_ptr(),
            tile_read_ready,
            core::ptr::null::<c_void>() as *mut c_void,
        );
        let e = psp::sys::sceIoSetAsyncCallback(fd, cb, core::ptr::null::<c_void>() as *mut c_void);
        if e < 0 {
            psp::dprintln!("e1: 0x{:x}", e);
        }
        let e = psp::sys::sceIoReadAsync(fd, DATA.as_mut_ptr() as *mut c_void, 16);
        if e < 0 {
            psp::dprintln!("e2: 0x{:x}", e);
        }

        psp::dprintln!("hi");

        let mut res = 0i64;
        // psp::sys::sceIoWaitAsync(fd, &mut res as *mut i64);
        // psp::dprintln!("res: 0x{:x}", res);
        psp::sys::sceIoWaitAsyncCB(fd, &mut res as *mut i64);
        psp::dprintln!("res: 0x{:x}", res);

        psp::dprintln!("fd: {}", fd.0);

        psp::sys::sceIoClose(fd);
    }
}

pub fn read_dir(path: &str) -> Vec<String> {
    let mut entries = Vec::new();

    unsafe {
        let fd = psp::sys::sceIoDopen(path.as_ptr());
        if fd.0 < 0 {
            psp::dprintln!(
                "error: 0x{:x} ({})",
                fd.0,
                match fd.0 as u32 {
                    IO_ERROR_NO_ENTRY => "File/directory does not exist",
                    _ => "unknown",
                }
            );
            return entries;
        }

        let mut entry = core::mem::uninitialized();
        while psp::sys::sceIoDread(fd, &mut entry) > 0 {
            let name = str::from_utf8(&entry.d_name)
                .unwrap_or("INVALID_UTF8")
                .to_string();
            entries.push(name);
        }

        psp::sys::sceIoDclose(fd);
    }

    entries
}
