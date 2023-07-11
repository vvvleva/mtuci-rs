use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use ptr::write;
use popptr::read;

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}


impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0);
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
}
fn main() {}

pub fn push<T>(&mut self, elem: T) {
    if self.len == self.cap { self.grow(); }

    unsafe {
        ptr::write(self.ptr.as_ptr().add(self.len), elem);
    }
    self.len += 1;
}

pub fn pop<T>(&mut elf) -> Option<T> {
    if self.len == 0 {
        None
    } else {
        self.len -= 1;
        unsafe {
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }
}
