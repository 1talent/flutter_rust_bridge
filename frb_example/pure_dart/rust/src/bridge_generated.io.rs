use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_simple_adder(port_: i64, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_primitive_types(
    port_: i64,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    wire_primitive_types_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_primitive_u32(port_: i64, my_u32: u32) {
    wire_primitive_u32_impl(port_, my_u32)
}

#[no_mangle]
pub extern "C" fn wire_handle_string(port_: i64, s: *mut wire_uint_8_list) {
    wire_handle_string_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_unit(port_: i64) {
    wire_handle_return_unit_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8(port_: i64, v: *mut wire_uint_8_list) {
    wire_handle_vec_u8_impl(port_, v)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_primitive(port_: i64, n: i32) {
    wire_handle_vec_of_primitive_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_vec_of_primitive(port_: i64, n: i32) {
    wire_handle_zero_copy_vec_of_primitive_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct(port_: i64, arg: *mut wire_MySize, boxed: *mut wire_MySize) {
    wire_handle_struct_impl(port_, arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype(port_: i64, arg: *mut wire_NewTypeInt) {
    wire_handle_newtype_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct(port_: i64, l: *mut wire_list_my_size) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_list(port_: i64, names: *mut wire_StringList) {
    wire_handle_string_list_impl(port_, names)
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct(port_: i64, s: *mut wire_MyTreeNode) {
    wire_handle_complex_struct_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_return(
    mode: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_handle_sync_return_impl(mode)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream(port_: i64, arg: *mut wire_uint_8_list) {
    wire_handle_stream_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_of_struct(port_: i64) {
    wire_handle_stream_of_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_err(port_: i64) {
    wire_return_err_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_panic(port_: i64) {
    wire_return_panic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return(port_: i64, left: f64, right: f64) {
    wire_handle_optional_return_impl(port_, left, right)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct(port_: i64, document: *mut wire_uint_8_list) {
    wire_handle_optional_struct_impl(port_, document)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment(port_: i64, opt: *mut wire_ExoticOptionals) {
    wire_handle_optional_increment_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_increment_boxed_optional(port_: i64, opt: *mut f64) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments(
    port_: i64,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_ExoticOptionals,
) {
    wire_handle_option_box_arguments_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[no_mangle]
pub extern "C" fn wire_print_note(port_: i64, note: *mut wire_Note) {
    wire_print_note_impl(port_, note)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_enum(port_: i64, input: *mut wire_uint_8_list) {
    wire_handle_return_enum_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_parameter(port_: i64, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[no_mangle]
pub extern "C" fn wire_handle_customized_struct(port_: i64, val: *mut wire_Customized) {
    wire_handle_customized_struct_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_struct(port_: i64, val: *mut wire_KitchenSink) {
    wire_handle_enum_struct_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_struct(port_: i64, my_struct: *mut wire_MyStruct) {
    wire_use_imported_struct_impl(port_, my_struct)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_enum(port_: i64, my_enum: i32) {
    wire_use_imported_enum_impl(port_, my_enum)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings(port_: i64) {
    wire_get_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded(port_: i64, app_settings: *mut wire_ApplicationSettings) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[no_mangle]
pub extern "C" fn wire_get_message(port_: i64) {
    wire_get_message_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_array(port_: i64) {
    wire_get_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_complex_array(port_: i64) {
    wire_get_complex_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_usize(port_: i64, u: usize) {
    wire_get_usize_impl(port_, u)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id(port_: i64, user_id: *mut wire_UserId) {
    wire_next_user_id_impl(port_, user_id)
}

#[no_mangle]
pub extern "C" fn wire_register_event_listener(port_: i64) {
    wire_register_event_listener_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_close_event_listener(port_: i64) {
    wire_close_event_listener_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_event(port_: i64) {
    wire_create_event_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_1(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_1_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_2(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_2_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_3(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_3_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_sum__method__SumWith(port_: i64, that: *mut wire_SumWith, y: u32, z: u32) {
    wire_sum__method__SumWith_impl(port_, that, y, z)
}

#[no_mangle]
pub extern "C" fn wire_new__static_method__ConcatenateWith(port_: i64, a: *mut wire_uint_8_list) {
    wire_new__static_method__ConcatenateWith_impl(port_, a)
}

#[no_mangle]
pub extern "C" fn wire_concatenate__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
    b: *mut wire_uint_8_list,
) {
    wire_concatenate__method__ConcatenateWith_impl(port_, that, b)
}

#[no_mangle]
pub extern "C" fn wire_concatenate_static__static_method__ConcatenateWith(
    port_: i64,
    a: *mut wire_uint_8_list,
    b: *mut wire_uint_8_list,
) {
    wire_concatenate_static__static_method__ConcatenateWith_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_stream_sink__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
    key: u32,
    max: u32,
) {
    wire_handle_some_stream_sink__method__ConcatenateWith_impl(port_, that, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_stream_sink_at_1__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
) {
    wire_handle_some_stream_sink_at_1__method__ConcatenateWith_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_static_stream_sink__static_method__ConcatenateWith(
    port_: i64,
    key: u32,
    max: u32,
) {
    wire_handle_some_static_stream_sink__static_method__ConcatenateWith_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith(
    port_: i64,
) {
    wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith_impl(port_)
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnv {
    vars: *mut wire_list_application_env_var,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnvVar {
    field0: *mut wire_uint_8_list,
    field1: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationSettings {
    name: *mut wire_uint_8_list,
    version: *mut wire_uint_8_list,
    mode: i32,
    env: *mut wire_ApplicationEnv,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Attribute {
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConcatenateWith {
    a: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Customized {
    final_field: *mut wire_uint_8_list,
    non_final_field: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ExoticOptionals {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_uint_8_list,
    int8list: *mut wire_int_8_list,
    uint8list: *mut wire_uint_8_list,
    int32list: *mut wire_int_32_list,
    float32list: *mut wire_float_32_list,
    float64list: *mut wire_float_64_list,
    attributes: *mut wire_list_attribute,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute,
    newtypeint: *mut wire_NewTypeInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_32_list {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_8_list {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_application_env_var {
    ptr: *mut wire_ApplicationEnvVar,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute {
    ptr: *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_MySize,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node {
    ptr: *mut wire_MyTreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute {
    ptr: *mut *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MySize {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyStruct {
    content: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyTreeNode {
    value_i32: i32,
    value_vec_u8: *mut wire_uint_8_list,
    value_boolean: bool,
    children: *mut wire_list_my_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NewTypeInt {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Note {
    day: *mut i32,
    body: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SumWith {
    x: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UserId {
    value: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink {
    tag: i32,
    kind: *mut KitchenSinkKind,
}

#[repr(C)]
pub union KitchenSinkKind {
    Empty: *mut KitchenSink_Empty,
    Primitives: *mut KitchenSink_Primitives,
    Nested: *mut KitchenSink_Nested,
    Optional: *mut KitchenSink_Optional,
    Buffer: *mut KitchenSink_Buffer,
    Enums: *mut KitchenSink_Enums,
}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Empty {}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Primitives {
    int32: i32,
    float64: f64,
    boolean: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Nested {
    field0: *mut wire_KitchenSink,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Optional {
    field0: *mut i32,
    field1: *mut i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Buffer {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct KitchenSink_Enums {
    field0: i32,
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_application_env_0() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings_0() -> *mut wire_ApplicationSettings {
    support::new_leak_box_ptr(wire_ApplicationSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute_0() -> *mut wire_Attribute {
    support::new_leak_box_ptr(wire_Attribute::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_concatenate_with_0() -> *mut wire_ConcatenateWith {
    support::new_leak_box_ptr(wire_ConcatenateWith::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_customized_0() -> *mut wire_Customized {
    support::new_leak_box_ptr(wire_Customized::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals_0() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kitchen_sink_0() -> *mut wire_KitchenSink {
    support::new_leak_box_ptr(wire_KitchenSink::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size_0() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_struct_0() -> *mut wire_MyStruct {
    support::new_leak_box_ptr(wire_MyStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node_0() -> *mut wire_MyTreeNode {
    support::new_leak_box_ptr(wire_MyTreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int_0() -> *mut wire_NewTypeInt {
    support::new_leak_box_ptr(wire_NewTypeInt::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note_0() -> *mut wire_Note {
    support::new_leak_box_ptr(wire_Note::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with_0() -> *mut wire_SumWith {
    support::new_leak_box_ptr(wire_SumWith::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id_0() -> *mut wire_UserId {
    support::new_leak_box_ptr(wire_UserId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals_0() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i8_0(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_kitchen_sink_0() -> *mut wire_KitchenSink {
    support::new_leak_box_ptr(wire_KitchenSink::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_my_size_0() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_weekdays_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_float_32_list_0(len: i32) -> *mut wire_float_32_list {
    let ans = wire_float_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_float_64_list_0(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_8_list_0(len: i32) -> *mut wire_int_8_list {
    let ans = wire_int_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var_0(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: support::new_leak_vec_ptr(<wire_ApplicationEnvVar>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_attribute_0(len: i32) -> *mut wire_list_attribute {
    let wrap = wire_list_attribute {
        ptr: support::new_leak_vec_ptr(<wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_size_0(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: support::new_leak_vec_ptr(<wire_MySize>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node_0(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_MyTreeNode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute_0(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute {
    let wrap = wire_list_opt_box_autoadd_attribute {
        ptr: support::new_leak_vec_ptr(<*mut wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<ApplicationEnv> for wire_ApplicationEnv {
    fn wire2api(self) -> ApplicationEnv {
        ApplicationEnv {
            vars: self.vars.wire2api(),
        }
    }
}
impl Wire2Api<ApplicationEnvVar> for wire_ApplicationEnvVar {
    fn wire2api(self) -> ApplicationEnvVar {
        ApplicationEnvVar(self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<ApplicationSettings> for wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        ApplicationSettings {
            name: self.name.wire2api(),
            version: self.version.wire2api(),
            mode: self.mode.wire2api(),
            env: self.env.wire2api(),
        }
    }
}
impl Wire2Api<Attribute> for wire_Attribute {
    fn wire2api(self) -> Attribute {
        Attribute {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}

impl Wire2Api<Box<ApplicationEnv>> for *mut wire_ApplicationEnv {
    fn wire2api(self) -> Box<ApplicationEnv> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApplicationSettings> for *mut wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationSettings>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Attribute> for *mut wire_Attribute {
    fn wire2api(self) -> Attribute {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Attribute>::wire2api(*wrap).into()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<ConcatenateWith> for *mut wire_ConcatenateWith {
    fn wire2api(self) -> ConcatenateWith {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ConcatenateWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Customized> for *mut wire_Customized {
    fn wire2api(self) -> Customized {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Customized>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ExoticOptionals> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ExoticOptionals>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<KitchenSink> for *mut wire_KitchenSink {
    fn wire2api(self) -> KitchenSink {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KitchenSink>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MySize> for *mut wire_MySize {
    fn wire2api(self) -> MySize {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyStruct> for *mut wire_MyStruct {
    fn wire2api(self) -> MyStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyTreeNode> for *mut wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyTreeNode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NewTypeInt> for *mut wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NewTypeInt>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Note> for *mut wire_Note {
    fn wire2api(self) -> Note {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Note>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SumWith> for *mut wire_SumWith {
    fn wire2api(self) -> SumWith {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SumWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<UserId> for *mut wire_UserId {
    fn wire2api(self) -> UserId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UserId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<bool>> for *mut bool {
    fn wire2api(self) -> Box<bool> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<ExoticOptionals>> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> Box<ExoticOptionals> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ExoticOptionals>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<f64>> for *mut f64 {
    fn wire2api(self) -> Box<f64> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i32>> for *mut i32 {
    fn wire2api(self) -> Box<i32> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i64>> for *mut i64 {
    fn wire2api(self) -> Box<i64> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i8>> for *mut i8 {
    fn wire2api(self) -> Box<i8> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<KitchenSink>> for *mut wire_KitchenSink {
    fn wire2api(self) -> Box<KitchenSink> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KitchenSink>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<MySize>> for *mut wire_MySize {
    fn wire2api(self) -> Box<MySize> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<u8>> for *mut u8 {
    fn wire2api(self) -> Box<u8> {
        unsafe { support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<Weekdays>> for *mut i32 {
    fn wire2api(self) -> Box<Weekdays> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Weekdays>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ConcatenateWith> for wire_ConcatenateWith {
    fn wire2api(self) -> ConcatenateWith {
        ConcatenateWith {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<Customized> for wire_Customized {
    fn wire2api(self) -> Customized {
        Customized {
            final_field: self.final_field.wire2api(),
            non_final_field: self.non_final_field.wire2api(),
        }
    }
}
impl Wire2Api<ExoticOptionals> for wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        ExoticOptionals {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}

impl Wire2Api<Vec<f32>> for *mut wire_float_32_list {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_int_8_list {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<KitchenSink> for wire_KitchenSink {
    fn wire2api(self) -> KitchenSink {
        match self.tag {
            0 => KitchenSink::Empty,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Primitives);
                KitchenSink::Primitives {
                    int32: ans.int32.wire2api(),
                    float64: ans.float64.wire2api(),
                    boolean: ans.boolean.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Nested);
                KitchenSink::Nested(ans.field0.wire2api(), ans.field1.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Optional);
                KitchenSink::Optional(ans.field0.wire2api(), ans.field1.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Buffer);
                KitchenSink::Buffer(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Enums);
                KitchenSink::Enums(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<ApplicationEnvVar>> for *mut wire_list_application_env_var {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Attribute>> for *mut wire_list_attribute {
    fn wire2api(self) -> Vec<Attribute> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<MySize> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<MyTreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<Attribute>>> for *mut wire_list_opt_box_autoadd_attribute {
    fn wire2api(self) -> Vec<Option<Attribute>> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<MySize> for wire_MySize {
    fn wire2api(self) -> MySize {
        MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<MyStruct> for wire_MyStruct {
    fn wire2api(self) -> MyStruct {
        MyStruct {
            content: self.content.wire2api(),
        }
    }
}
impl Wire2Api<MyTreeNode> for wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<NewTypeInt> for wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        NewTypeInt(self.field0.wire2api())
    }
}
impl Wire2Api<Note> for wire_Note {
    fn wire2api(self) -> Note {
        Note {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<Option<String>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Option<String> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<ZeroCopyBuffer<Vec<u8>>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Option<ZeroCopyBuffer<Vec<u8>>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Attribute>> for *mut wire_Attribute {
    fn wire2api(self) -> Option<Attribute> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<bool>> for *mut bool {
    fn wire2api(self) -> Option<bool> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<ExoticOptionals>> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> Option<ExoticOptionals> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<f64>> for *mut f64 {
    fn wire2api(self) -> Option<f64> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<i32>> for *mut i32 {
    fn wire2api(self) -> Option<i32> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<i64>> for *mut i64 {
    fn wire2api(self) -> Option<i64> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<NewTypeInt>> for *mut wire_NewTypeInt {
    fn wire2api(self) -> Option<NewTypeInt> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<bool>>> for *mut bool {
    fn wire2api(self) -> Option<Box<bool>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<ExoticOptionals>>> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> Option<Box<ExoticOptionals>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<f64>>> for *mut f64 {
    fn wire2api(self) -> Option<Box<f64>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i32>>> for *mut i32 {
    fn wire2api(self) -> Option<Box<i32>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i64>>> for *mut i64 {
    fn wire2api(self) -> Option<Box<i64>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i8>>> for *mut i8 {
    fn wire2api(self) -> Option<Box<i8>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<u8>>> for *mut u8 {
    fn wire2api(self) -> Option<Box<u8>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f32>>> for *mut wire_float_32_list {
    fn wire2api(self) -> Option<Vec<f32>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f64>>> for *mut wire_float_64_list {
    fn wire2api(self) -> Option<Vec<f64>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i32>>> for *mut wire_int_32_list {
    fn wire2api(self) -> Option<Vec<i32>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i8>>> for *mut wire_int_8_list {
    fn wire2api(self) -> Option<Vec<i8>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<Attribute>>> for *mut wire_list_attribute {
    fn wire2api(self) -> Option<Vec<Attribute>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<Option<Attribute>>>> for *mut wire_list_opt_box_autoadd_attribute {
    fn wire2api(self) -> Option<Vec<Option<Attribute>>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Option<Vec<u8>> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<SumWith> for wire_SumWith {
    fn wire2api(self) -> SumWith {
        SumWith {
            x: self.x.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<UserId> for wire_UserId {
    fn wire2api(self) -> UserId {
        UserId {
            value: self.value.wire2api(),
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_ApplicationEnv {
    fn new_with_null_ptr() -> Self {
        Self {
            vars: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ApplicationEnvVar {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_ApplicationSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            version: core::ptr::null_mut(),
            mode: Default::default(),
            env: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Attribute {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ConcatenateWith {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Customized {
    fn new_with_null_ptr() -> Self {
        Self {
            final_field: core::ptr::null_mut(),
            non_final_field: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ExoticOptionals {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: core::ptr::null_mut(),
            int64: core::ptr::null_mut(),
            float64: core::ptr::null_mut(),
            boolean: core::ptr::null_mut(),
            zerocopy: core::ptr::null_mut(),
            int8list: core::ptr::null_mut(),
            uint8list: core::ptr::null_mut(),
            int32list: core::ptr::null_mut(),
            float32list: core::ptr::null_mut(),
            float64list: core::ptr::null_mut(),
            attributes: core::ptr::null_mut(),
            attributes_nullable: core::ptr::null_mut(),
            nullable_attributes: core::ptr::null_mut(),
            newtypeint: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_KitchenSink {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Primitives() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Primitives: support::new_leak_box_ptr(KitchenSink_Primitives {
            int32: Default::default(),
            float64: Default::default(),
            boolean: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Nested() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Nested: support::new_leak_box_ptr(KitchenSink_Nested {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Optional() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Optional: support::new_leak_box_ptr(KitchenSink_Optional {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Buffer() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Buffer: support::new_leak_box_ptr(KitchenSink_Buffer {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Enums() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Enums: support::new_leak_box_ptr(KitchenSink_Enums {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_MySize {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            content: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyTreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: core::ptr::null_mut(),
            value_boolean: Default::default(),
            children: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_NewTypeInt {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Note {
    fn new_with_null_ptr() -> Self {
        Self {
            day: core::ptr::null_mut(),
            body: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_SumWith {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_UserId {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
