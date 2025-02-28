use super::env::get_emscripten_data;
use super::process::abort_with_message;
use libc::c_int;
// use std::cell::UnsafeCell;
use crate::EmEnv;
use std::error::Error;
use std::fmt;

use wasmer::RuntimeError;

/// setjmp
pub fn __setjmp(ctx: &EmEnv, _env_addr: u32) -> c_int {
    debug!("emscripten::__setjmp (setjmp)");
    abort_with_message(ctx, "missing function: _setjmp");
    unreachable!()
    // unsafe {
    //     // Rather than using the env as the holder of the jump buffer pointer,
    //     // we use the environment address to store the index relative to jumps
    //     // so the address of the jump it's outside the wasm memory itself.
    //     let jump_index = emscripten_memory_pointer!(ctx.memory(0), env_addr) as *mut i8;
    //     // We create the jump buffer outside of the wasm memory
    //     let jump_buf: UnsafeCell<[u32; 27]> = UnsafeCell::new([0; 27]);
    //     let jumps = &mut get_emscripten_data(ctx).jumps;
    //     let result = setjmp(jump_buf.get() as _);
    //     // We set the jump index to be the last 3value of jumps
    //     *jump_index = jumps.len() as _;
    //     // We hold the reference of the jump buffer
    //     jumps.push(jump_buf);
    //     result
    // }
}

/// longjmp
#[allow(unreachable_code)]
pub fn __longjmp(ctx: &EmEnv, _env_addr: u32, _val: c_int) {
    debug!("emscripten::__longjmp (longmp)");
    abort_with_message(ctx, "missing function: _longjmp");
    // unsafe {
    //     // We retrieve the jump index from the env address
    //     let jump_index = emscripten_memory_pointer!(ctx.memory(0), env_addr) as *mut i8;
    //     let jumps = &mut get_emscripten_data(ctx).jumps;
    //     // We get the real jump buffer from the jumps vector, using the retrieved index
    //     let jump_buf = &jumps[*jump_index as usize];
    //     longjmp(jump_buf.get() as _, val)
    // };
}

#[derive(Copy, Clone, Debug)]
pub struct LongJumpRet;

impl fmt::Display for LongJumpRet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LongJumpRet")
    }
}

impl Error for LongJumpRet {}

/// _longjmp
// This function differs from the js implementation, it should return Result<(), &'static str>
#[allow(unreachable_code)]
pub fn _longjmp(ctx: &EmEnv, env_addr: i32, val: c_int) -> Result<(), RuntimeError> {
    let val = if val == 0 { 1 } else { val };
    get_emscripten_data(ctx)
        .set_threw_ref()
        .expect("set_threw is None")
        .call(env_addr, val)
        .expect("set_threw failed to call");
    Err(RuntimeError::custom(Box::new(LongJumpRet)))
}

// extern "C" {
//     fn setjmp(env: *mut c_void) -> c_int;
//     fn longjmp(env: *mut c_void, val: c_int) -> !;
// }
