use super::*;

#[derive(Debug)]
#[allow(raw_pointer_derive)]
pub struct Stack {
    capacity: usize,
    size: usize,
    ptr: *mut Value,
    pushes: u64,
    pops: u64,
}

impl Stack {
    pub fn new() -> Stack {
        use std::mem::{transmute, size_of};
        use libc::malloc;
        let capacity = 1000;
        let allocated_size = size_of::<Value>() * capacity;
        let ptr = unsafe { transmute(malloc(allocated_size)) };

        Stack {
            capacity: capacity,
            size: 0,
            ptr: ptr,
            pushes: 0,
            pops: 0,
        }
    }

    pub fn push_count(&self) -> u64 {
        self.pushes
    }

    pub fn pop_count(&self) -> u64 {
        self.pops
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<Value, InterpError> {
        use std::mem::{uninitialized, swap, transmute};
        self.pops += 1;
        if self.size == 0 {
            return Err(InterpError::StackUnderflow);
        }

        unsafe {
            let mut out = uninitialized();
            self.size -= 1;
            swap(&mut out, transmute::<*mut _, &mut _>(self.ptr.offset(self.size as isize)));
            Ok(out)
        }
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Result<&mut Value, InterpError> {
        use std::mem::transmute;
        if self.size == 0 {
            return Err(InterpError::StackOutOfBounds);
        }

        unsafe {
            Ok(transmute::<*mut _, &mut _>(self.ptr.offset(self.size as isize - 1)))
        }
    }

    #[inline(always)]
    pub fn peek_n(&mut self, n: usize) -> Result<&mut Value, InterpError> {
        use std::mem::transmute;
        if n >= self.size {
            return Err(InterpError::StackOutOfBounds);
        }

        unsafe {
            Ok(transmute::<*mut _, &mut _>(self.ptr.offset(self.size as isize - n as isize - 1)))
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: Value) -> Result<(), InterpError> {
        use std::ptr::write;
        self.pushes += 1;
        if self.size == self.capacity {
            return Err(InterpError::StackOverflow);
        }

        unsafe {
            write(self.ptr.offset(self.size as isize), value);
        }
        self.size += 1;
        Ok(())
    }

    pub fn len(&self) -> u32 {
        self.size as u32
    }

    #[inline(always)]
    pub fn swap(&mut self, a: u32, b: u32) -> Result<(), InterpError> {
        use std::ptr::swap;
        if a as usize >= self.size || b as usize >= self.size {
            return Err(InterpError::StackOutOfBounds);
        }

        if a != b {
            unsafe {
                let ptr_a = self.ptr.offset(a as isize);
                let ptr_b = self.ptr.offset(b as isize);
                swap(ptr_a, ptr_b);
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub fn binop_int<F: FnOnce(i64, i64) -> i64>(&mut self, f: F) -> Result<(), InterpError> {
        let a = try!(try!(self.pop()).expect_int());
        let b = try!(try!(self.peek()).expect_int_ref_mut());
        *b = f(*b, a);
        Ok(())
    }
}
