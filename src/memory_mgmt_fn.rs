#[allow(non_snake_case)]
use std::ptr::copy;

pub(crate) unsafe fn write (dst : *mut u8, src : &[u8]) -> () {

    copy (src.as_ptr(),
         dst,
         src.len());

}

pub(crate) unsafe fn read (dst : &mut [u8], src : *mut u8, len : usize) -> () {

    copy (src,
          dst.as_mut_ptr(),
          len);

}

pub(crate) unsafe fn add (base : *mut u8, relative_addr : usize) -> *mut u8 {

    base.add (relative_addr)

}
