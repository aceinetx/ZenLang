use alloc::vec::*;
use core::{
    alloc::Layout,
    ops::{Index, IndexMut},
};

/// RawVec
///
/// This is a vector with shared memory, this means if you clone/copy it, it won't copy the memory and the addresses will stay the same
/// This is extremely unsafe, only used in Object and should not be used anywhere outside zenlang
#[derive(Clone, Copy)]
pub struct RawVec<T> {
    pub items: *mut T,
    pub len: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> RawVec<T> {
        let instance = RawVec {
            items: 0 as *mut T,
            len: 0,
        };
        return instance;
    }

    /// Get a layout for N length
    pub(crate) fn layout_for_len(&self, len: usize) -> Layout {
        return Layout::array::<T>(len).unwrap();
    }

    /// Free the internal items buffer
    /// Does not modify self.len
    pub fn dealloc(&mut self) {
        if self.items != 0 as *mut T {
            if self.len == 0 {
                panic!("rwvec: self.len or self.capacity are zero - this vector cannot exist");
            }

            unsafe {
                alloc::alloc::dealloc(self.items as *mut u8, self.layout_for_len(self.len));
            }
            self.items = 0 as *mut T;
        }
    }

    /// Same as dealloc
    pub fn clear(&mut self) {
        self.dealloc();
        self.len = 0;
    }

    /// Allocate N more elements
    /// DOES NOT change self.len
    pub(crate) unsafe fn grow(&mut self, count: usize) {
        if self.items == 0 as *mut T {
            unsafe {
                self.items = alloc::alloc::alloc(self.layout_for_len(count)) as *mut T;
            }
        } else {
            unsafe {
                self.items = alloc::alloc::realloc(
                    self.items as *mut u8,
                    self.layout_for_len(self.len),
                    self.layout_for_len(self.len + count).size(),
                ) as *mut T;
            }
        }
    }

    /// Deallocate N elements
    /// DOES NOT change self.len
    pub(crate) unsafe fn shrink(&mut self, count: usize) {
        if self.items == 0 as *mut T {
            panic!("called shrink on an empty vector")
        } else {
            if count > self.len {
                self.dealloc();
                panic!("shrink count is more than there are elements in the vec")
            }
            if count == self.len {
                self.dealloc();
                return;
            }

            unsafe {
                self.items = alloc::alloc::realloc(
                    self.items as *mut u8,
                    self.layout_for_len(self.len),
                    self.layout_for_len(self.len - count).size(),
                ) as *mut T;
            }
        }
    }

    pub fn push(&mut self, e: T) {
        unsafe {
            self.grow(1);
            *self.items.offset(self.len as isize) = e;

            self.len += 1;
        }
    }

    pub fn pop(&mut self) {
        unsafe {
            assert!(self.len != 0, "pop on an empty vector");

            self.shrink(1);

            self.len -= 1;
        }
    }

    pub fn last(&self) -> &T {
        return self.at(self.len - 1);
    }

    pub fn last_mut(&mut self) -> &mut T {
        return self.at_mut(self.len - 1);
    }

    pub fn at(&self, index: usize) -> &T {
        unsafe {
            return self.items.offset(index as isize).as_mut().unwrap();
        }
    }

    pub fn at_mut(&mut self, index: usize) -> &mut T {
        unsafe {
            return self.items.offset(index as isize).as_mut().unwrap();
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.len {
            self.dealloc();
            panic!("remove in an out of bounds index");
        }

        self.len -= 1;

        unsafe {
            core::ptr::copy(
                self.items.add(index + 1),
                self.items.add(index),
                self.len - index,
            );
        }
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn from_regular(vec: &Vec<T>) -> RawVec<T> {
        let mut inst: RawVec<T> = RawVec::new();
        for i in vec.iter() {
            unsafe {
                inst.push(core::ptr::read(i));
            }
        }
        return inst;
    }

    pub fn iter(&self) -> RawValIter<T> {
        unsafe {
            return RawValIter {
                start: self.items,
                end: self.items.add(self.len),
            };
        }
    }
}

pub struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let old_ptr = self.start;
                self.start = self.start.offset(1);
                Some(core::ptr::read(old_ptr))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = core::mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(core::ptr::read(self.end))
            }
        }
    }
}

impl<T> Index<usize> for RawVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return self.at(index);
    }
}

impl<T> IndexMut<usize> for RawVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.at_mut(index);
    }
}
