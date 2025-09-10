// pld - src/mmm.rs (manual memory management helper)
// ----------------
// made by matissoss
// licensed under MPL 2.0

use std::alloc;

// because fuck builtin rust's safety
// and i won't write it in C, because i'm too lazy to write actual linker in C (maybe someday?)

// WHAT TF IS THIS 
pub unsafe fn memcpy<T>(src: *const T, dst: *mut T, sz: usize) {
    unsafe {
        let src = std::mem::transmute::<*const T, *const u8>(src);
        let dst = std::mem::transmute::<*mut T, *mut u8>(dst);
        __memcpy(src, dst, sz);
    }
}

// another recursive unsafe :D
pub unsafe fn __memcpy(src: *const u8, dst: *mut u8, sz: usize) {
    unsafe {
        let mut idx = 0;
        while idx < sz {
            *dst.add(idx) = *src.add(idx);
            idx += 1;
        }
    }
}

pub unsafe fn malloc<T>(sz: usize, align: usize) -> *mut T {
    unsafe {
        let ptr = __malloc(sz, align);
        std::mem::transmute::<*mut u8, *mut T>(ptr)
    }
}

pub unsafe fn __malloc(sz: usize, align: usize) -> *mut u8 {
    unsafe {
        let layout = alloc::Layout::from_size_align(sz, align)
            .expect("mmm.rs:__malloc something went wrong idk lol");
        alloc::alloc(layout)
    }
}

