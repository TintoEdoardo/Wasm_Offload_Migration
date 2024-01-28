#[allow(non_snake_case)]
mod memory_mgmt_fn;

use crate::memory_mgmt_fn::{add, read, write};
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> wasmtime::Result<()> {

    /*  Define a WASM module with the associated linear memory.
        This step requires to define an Engine and a Store.     */
    let engine = Engine::default ();
    let mut store = Store::new (&engine, ());
    let module = Module::from_file (&engine, "./src/wasm_module/lib.wasm")?;
    let instance = Instance::new (&mut store, &module, &[])?;

    /*  Write in the linear memory of the module.
        To do so we need to allocate a memory segment from within the
        Wasm module. Then, we can place content within this memory.
        The 'allocate' function is defined within the Wasm module,
        and its implementation might differ depending on the system.  */
    let allocate = instance
        .get_typed_func::<i32, i32> (&mut store, "alloc")
        .expect("Function 'alloc' retrieval failed");

    let memory = instance
        .get_memory (&mut store, "memory")
        .expect ("Memory not found");

    /*  The content we intend to write in the linear memory.  */
    let content = [1, 2, 3];
    let len = content.len() as i32;

    /*  The call to 'allocate' within the Wasm module returns the
        relative address of the allocated memory segment within
        the Wasm module linear memory.  */
    let relative_addr = allocate.call (&mut store, len)?;

    /*  To access to the allocated memory segment within the module
        linear memory, we need the address of this segment, from
        the host perspective. */
    let ptr = unsafe {
        add (memory.data_ptr (&mut store), relative_addr as usize)
    };

    /*  Write the content of 'content' within the linear memory.  */
    unsafe {
        write (ptr, content.as_ref ());
    }

    /*  Execute the multiplier function within the Wasm module.  */
    let multiplier_function = instance
        .get_typed_func::<(i32, i32), i32> (&mut store, "multiplier_function")
        .expect("Function 'multiplier_func' not found");

    /*  Note that we pass the relative address to the Wasm module.  */
    let result = multiplier_function
        .call (&mut store, (relative_addr, len))
        .expect("Call to function 'multiplier_func' failed");

    assert_eq!(result, 6);

    /*  Copy the content of the allocated segment within the
        linear memory into a buffer.  */
    let mut buffer = [0u8, 0, 0];
    let buffer_ref = buffer.as_mut ();
    unsafe {
        read (buffer_ref, ptr, len as usize);
    }

    assert_eq! (content [0], buffer [0]);
    assert_eq! (content [1], buffer [1]);
    assert_eq! (content [2], buffer [2]);

    Ok(())

}