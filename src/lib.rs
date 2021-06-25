#![no_std]

use core::{mem::MaybeUninit};

#[macro_export]
macro_rules! copy_vec {
    ($array_type:ty, $n:literal => $($elem:expr),* $(,)?) => {
        {
            let mut cv: $crate::CopyVec<$array_type, $n> = Default::default();
            $( cv.push($elem); )*
            cv
        }
    };
    ($n:literal => $($elem:expr),* $(,)?) => {
        $crate::copy_vec!(_, $n => $($elem),*)
    }
}

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

    pub fn as_slice(&self) -> &[T] {
        self.unsafe_slice()
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn remaining_capacity(&self) -> usize {
        N.saturating_sub(self.len)
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.len = core::cmp::min(new_len, self.len);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let elem = unsafe { self.array[self.len].assume_init() };
            self.len -= 1;
            Some(elem)
        } else {
            None
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            None
        } else {
            let value = unsafe { self.array[index].assume_init() }; 

            let remaining_to_copy = self.len - index - 1;
            unsafe {
                core::ptr::copy(
                    self.array.as_ptr().add(index + 1),
                    self.array.as_mut_ptr().add(index),
                    remaining_to_copy
                );
            }
            self.len -= 1;
            Some(value)
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self.pop_at(index) {
            Some(v) => v,
            None => panic!("trying to remove index {} of copyvec with length {}", index, self.len),
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, value: T) {
        if let Err(_) = self.try_push(value) {
            panic!("copyvec out of capacity")
        }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            Err(value)
        } else {
            unsafe {
                self.array[self.len].as_mut_ptr().write(value);
            }
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

impl<T: Copy, const N: usize> AsMut<[T]> for CopyVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.unsafe_mut_slice()
    }
}

impl<T: Copy, const N: usize> AsRef<[T]> for CopyVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self.unsafe_slice()
    }
}

impl<T: Copy, const N: usize> core::borrow::BorrowMut<[T]> for CopyVec<T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.unsafe_mut_slice()
    }
}

impl<T: Copy, const N: usize> core::borrow::Borrow<[T]> for CopyVec<T, N> {
    fn borrow(&self) -> &[T] {
        self.unsafe_slice()
    }
}

impl<T: Copy, const N: usize> core::cmp::PartialEq for CopyVec<T, N> where T: core::cmp::PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T: Copy, const N: usize> core::cmp::Eq for CopyVec<T, N> where T: core::cmp::Eq {}

// include when https://github.com/rust-lang/rust/issues/80908 is stable
// impl<T: Copy, const N: usize> From<[T; N]> for CopyVec<T, N> {
//     fn from(data: [T; N]) -> Self {
//         Self {
//             array: unsafe { core::mem::transmute::<[T; N], [MaybeUninit<T>; N]>(data) },
//             len: N
//         }
//     }
// }

impl<T: Copy, const N: usize> core::fmt::Debug for CopyVec<T, N> where T: core::fmt::Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list()
            .entries(self.iter())
            .finish()
    }
}

#[cfg(feature = "serde")]
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer, Error as DeserializeError, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};

/* Serde parts */
#[cfg(feature = "serde")]
#[cfg_attr(docs_rs, doc(cfg(feature = "serde")))]
impl<T: Copy, const N: usize> Serialize for CopyVec<T, N> where T: Serialize {
    #[must_use]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.len))?;
        for element in self.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docs_rs, doc(cfg(feature = "serde")))]
impl<'de, T: Copy, const N: usize> Deserialize<'de> for CopyVec<T, N> where T: Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_seq(CopyVecVisitor::<T, N>(PhantomData))
    }
}


#[cfg(feature = "serde")]
struct CopyVecVisitor<T: Copy, const N: usize>(PhantomData<T>);

#[cfg(feature = "serde")]
impl<'de, T: Copy, const N: usize> Visitor<'de> for CopyVecVisitor<T, N> where T: Deserialize<'de> {
    type Value = CopyVec<T, N>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "a sequence with at most {} elements", N)
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error> where S: SeqAccess<'de> {
        let mut new_copyvec: CopyVec<T, N> = Default::default();

        let mut cur_len = 0usize; // current length tracked in case of error
        while let Some(value) = seq.next_element()? {
            if let Err(_) = new_copyvec.try_push(value) {
                let wanted_len = seq.size_hint().unwrap_or(cur_len);
                return Err(DeserializeError::invalid_length(wanted_len, &self));
            } else {
                cur_len = cur_len + 1;
            }
        }

        Ok(new_copyvec)
    }
}