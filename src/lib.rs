#![no_std]

use core::mem::MaybeUninit;

pub struct CopyVec<T: Copy, const N: usize> {
    len: usize,
    array: [MaybeUninit<T>; N],
}

impl<T: Copy, const N: usize> CopyVec<T, N> {
    pub fn new() -> Self {
        Self {
            len: 0,
            array: [MaybeUninit::uninit(); N]
        }
    }

    fn unsafe_slice(&self) -> &[T] {
        unsafe {
            core::mem::transmute(&self.array[0..self.len])
        }
    }

    fn unsafe_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            core::mem::transmute(&mut self.array[0..self.len])
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut [T] {
        self.unsafe_mut_slice() as *mut [T]
    }

    pub fn as_ptr(&self) -> *const [T] {
        self.unsafe_slice() as *const [T]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.unsafe_mut_slice()
    }

    pub fn as_slice(&mut self) -> &[T] {
        self.unsafe_slice()
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, value: T) {
        self.try_push(value).expect("copyvec out of capacity")
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            Err(T)
        } else {
            self.array[self.len].write(value);
            self.len += 1;
            Ok(())
        }
    }
}

impl<T: Copy, const N: usize> core::ops::Deref for CopyVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.unsafe_slice()
    }
}

impl<T: Copy, const N: usize> core::ops::DerefMut for CopyVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.unsafe_mut_slice()
    }
}

impl<T: Copy, const N: usize> Default for CopyVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}