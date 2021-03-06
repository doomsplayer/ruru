use ruby_sys::class;

use binding::util as binding_util;
use class::traits::Object;
use types::{Argc, Callback, CallbackPtr, Value};
use util;

pub fn define_class(name: &str, superclass: Value) -> Value {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_class(name.as_ptr(), superclass) }
}

pub fn object_class(object: Value) -> Value {
    unsafe { class::rb_obj_class(object) }
}

pub fn new_instance(klass: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe { class::rb_class_new_instance(argc, argv, klass) }
}

pub fn instance_variable_get(object: Value, name: &str) -> Value {
    unsafe { class::rb_ivar_get(object, binding_util::internal_id(name)) }
}

pub fn instance_variable_set(object: Value, name: &str, value: Value) -> Value {
    unsafe { class::rb_ivar_set(object, binding_util::internal_id(name), value) }
}

pub fn define_method<I: Object, O: Object>(klass: Value, name: &str, callback: Callback<I, O>) {
    let name = util::str_to_cstring(name);

    unsafe {
        class::rb_define_method(klass, name.as_ptr(), callback as CallbackPtr, -1);
    }
}

pub fn define_singleton_method<I: Object, O: Object>(klass: Value,
                                                     name: &str,
                                                     callback: Callback<I, O>) {
    let name = util::str_to_cstring(name);

    unsafe {
        class::rb_define_singleton_method(klass, name.as_ptr(), callback as CallbackPtr, -1);
    }
}
