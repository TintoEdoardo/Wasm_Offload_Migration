/*  The function contained in the Wasm module                               */
/*  Inspired by https://radu-matei.com/blog/practical-guide-to-wasm-memory/ */
/*  and https://petermalmgren.com/serverside-wasm-data/                     */

/*  #[link(wasm_import_module = "host")]
extern {
    fn get_ptr () -> *mut u8;
    fn get_len () -> usize;
}  */

use std::mem;
use std::slice;
use std::os::raw::c_void;

/*  Allocate a memory segment within the module linear memory  */
#[no_mangle]
pub extern fn alloc (size: usize) -> *mut c_void {

    let mut buf = Vec::with_capacity (size);
    let ptr = buf.as_mut_ptr ();

    /*  Prevent deallocation on return  */
    mem::forget (buf);

    return ptr as *mut c_void;

}

#[no_mangle]
pub unsafe fn multiplier_function (ptr: *mut u8, len: usize) -> u8 {

    let slice = unsafe {
        slice::from_raw_parts (ptr as _, len as _)
    };

    /*  Compute the product of the element in memory
        let result = memory_content.iter().product();  */
    let result = slice.iter().product();

    return result;

}