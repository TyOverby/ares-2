use super::*;
pub struct Stack {
    capacity: usize,
    size: usize,
    ptr: *mut Value,
    pushes: u64,
    pops: u64,
}

impl ::std::fmt::Debug for Stack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        try!(writeln!(f, "Stack"));
        try!(writeln!(f, "-----"));
        for v in self.as_slice() {
            try!(writeln!(f, "{:?}", v));
        }
        try!(writeln!(f, "-----"));
        Ok(())
    }
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

    pub fn take_top(&mut self, n: u32) -> Result<Vec<Value>, InterpError> {
        let mut v = vec![];
        for _ in 0 .. n {
            v.push(try!(self.pop()));
        }
        v.reverse();
        Ok(v)
    }

    pub fn push_count(&self) -> u64 {
        self.pushes
    }

    pub fn pop_count(&self) -> u64 {
        self.pops
    }

    pub fn truncate(&mut self, size: usize) -> Result<(), InterpError> {
        if self.size >= size {
            self.size = size;
            Ok(())
        } else {
            Err(InterpError::InternalInterpError(format!("stack.truncate({}) with a current \
                                                          size of {}",
                                                         size,
                                                         self.size)))
        }
    }

    #[inline(always)]
    pub fn keep(&mut self, n: u32) -> Result<Vec<Value>, InterpError> {
        // 0 1 2 3 4 5 | len = 6
        //       ^     | n = 3
        //             | number_to_pop = 3
        println!("len: {}, n: {}", self.len(), n);
        let number_to_pop = self.len() - n;
        self.pop_n(number_to_pop as usize)
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
            swap(&mut out,
                 transmute::<*mut _, &mut _>(self.ptr.offset(self.size as isize)));
            Ok(out)
        }
    }

    #[inline(always)]
    pub fn pop_n(&mut self, n: usize) -> Result<Vec<Value>, InterpError> {
        if self.size < n {
            return Err(InterpError::StackUnderflow);
        }
        if n == 0 {
            return Ok(vec![]);
        }

        let v = {
            let slice = self.as_slice();
            let (_, above) = slice.split_at(self.size - n);
            let v: Vec<Value> = Vec::from(above);
            v
        };
        self.size -= n;
        Ok(v)
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Result<&mut Value, InterpError> {
        use std::mem::transmute;
        if self.size == 0 {
            return Err(InterpError::StackOutOfBounds);
        }

        unsafe { Ok(transmute::<*mut _, &mut _>(self.ptr.offset(self.size as isize - 1))) }
    }

    #[inline(always)]
    pub fn peek_n_up(&mut self, n: usize) -> Result<&mut Value, InterpError> {
        use std::mem::transmute;
        if n >= self.size {
            return Err(InterpError::StackOutOfBounds);
        }

        unsafe { Ok(transmute::<*mut _, &mut _>(self.ptr.offset(n as isize))) }
    }

    #[inline(always)]
    pub fn peek_n_down(&mut self, n: usize) -> Result<&mut Value, InterpError> {
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
        let b = try!(try!(self.peek()).expect_int_mut());
        *b = f(*b, a);
        Ok(())
    }

    pub fn as_slice(&self) -> &[Value] {
        unsafe { ::std::slice::from_raw_parts(self.ptr, self.size) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Value] {
        unsafe { ::std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}
