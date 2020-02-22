#[macro_use]
extern crate safe_uninit_derive;

mod safe_uninit {
    pub unsafe trait SafeUninit: Sized {

        fn safe_uninit() -> Self {
            unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            }
        }
    }

    unsafe impl SafeUninit for usize {}
}

#[derive(SafeUninit)]
struct Test3(usize, isize);

pub fn main() {}
