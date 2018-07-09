use rubysys::rproc;

use binding::global::RubySpecialConsts;
use types::{InternalValue, Value, Callback, CallbackPtr, CallbackMutPtr};
use ::Object;
use util;

pub fn new<F, R>(func: F, binding: Value) -> Value
  where F: 'static + FnOnce() -> R, R: Object {
    let fnbox = Box::new(func) as Box<FnOnce() -> R>;
    let closure_ptr = Box::into_raw(Box::new(fnbox)) as CallbackMutPtr;

    unsafe {
        rproc::rb_iterate(rproc::rb_block_proc, Value::from(0), closure_ptr, binding)
        //rproc::rb_proc_new(closure_ptr, binding)
    }
}

pub fn call(rproc: Value, arguments: Option<Vec<Value>>) -> Value {
    let (argc, argv) = util::process_arguments(&arguments);

    unsafe {
        rproc::rb_proc_call_with_block(
            rproc,
            argc,
            argv,
            Value::from(RubySpecialConsts::Nil as InternalValue),
        )
    }
}

// extern "C" fn proc_create_callbox<R>(boxptr: *mut c_void) -> Value
// where
//     R: Object,
// {
//     let mut fnbox: Box<Box<FnMut() -> R>> =
//         unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> R>) };
// 
//     fnbox().value()
// }
