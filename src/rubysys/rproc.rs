use rubysys::types::{Argc, Value, CallbackMutPtr, c_void};

extern "C" {
    pub fn rb_proc_call_with_block(rproc: Value,
                                   argc: Argc,
                                   argv: *const Value,
                                   pass_procval: Value)
                                   -> Value;
    // TODO: FIXME
    pub fn rb_proc_s_new(argc: Argc, argv: Value, klass: Value) -> Value;
    //pub fn rb_block_proc() -> Value;
    // `rb_iterate` preferred over `rb_proc_new` because we need to use a Rust local
    // callback pointer for the return value.
    //pub fn rb_proc_new(func: CallbackPtr, binding: Value) -> Value;
    //pub fn rb_iterate(ret: unsafe extern "C" fn() -> Value, data1: Value, func: CallbackMutPtr, binding: Value) -> Value;
    // TODO (a few easy methods)
    // pub fn rb_obj_is_proc(obj: Value) -> Value;
    // pub fn rb_obj_is_method(obj: Value) -> Value;
    // pub fn rb_method_location(obj: Value) -> Value;
}
