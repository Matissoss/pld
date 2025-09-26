// pld - src/mmm.rs
// ----------------
// made by matissoss
// licensed under MPL 2.0

pub trait FromPtr {
    unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a Self;
}

impl<T> FromPtr for T {
    unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a Self {
        unsafe {
            let ptr = std::mem::transmute::<*const u8, *const Self>(ptr);
            &*ptr
        }
    }
}
