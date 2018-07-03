use rubysys::types::{Argc, Value, CallbackPtr};

extern "C" {
    pub fn rb_proc_call_with_block(rproc: Value,
                                   argc: Argc,
                                   argv: *const Value,
                                   pass_procval: Value)
                                   -> Value;
    pub fn rb_proc_new(func: CallbackPtr, binding: Value) -> Value;
    // TODO (a few easy methods)
    // pub fn rb_obj_is_proc(obj: Value) -> Value;
    // pub fn rb_obj_is_method(obj: Value) -> Value;
    // pub fn rb_method_location(obj: Value) -> Value;
}
