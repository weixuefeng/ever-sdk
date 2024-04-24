pub mod dart_types;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};

extern crate ton_client;
pub use ton_client::{create_context, destroy_context, request};

#[no_mangle]
static mut NATIVE_PORT:i64 = -1;

#[no_mangle]
static mut POST_C_OBJECT: Option<dart_types::DartPostCObjectFnType> = None;

#[no_mangle]
pub unsafe extern "C" fn dart_create_context(port: i64, config: *const c_char) -> *mut c_char {
    NATIVE_PORT = port;
    let c_str : &CStr = CStr::from_ptr(config);
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    let res = create_context(str_buf);
    return  CString::new(res.as_str()).unwrap().into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn dart_destroy_context(context: u32)  {
    NATIVE_PORT = -1;
    destroy_context(context);
}

#[no_mangle]
pub unsafe extern "C" fn dart_request(context: u32,function_name: *const c_char, function_params_json: *const c_char, request_id: u32){
    let str_function_name: &str = CStr::from_ptr(function_name).to_str().unwrap();
    let str_params_json: &str = CStr::from_ptr(function_params_json).to_str().unwrap();
    request(
        context,
        str_function_name.to_owned(),
        str_params_json.to_owned(),
        request_id,
        dart_response_handler,
    )
}

fn dart_response_handler(request_id: u32, params_json: String, response_type: u32, finished: bool)
{
    let response = Box::into_raw(Box::new(DartResponse{
        request_id: request_id,
        params_json: CString::new(params_json.as_str()).unwrap().into_raw(),
        response_type: response_type,
        finished: finished,
    }));
    post_object(response);
}

#[repr(C)]
#[derive(Clone)]
pub struct DartResponse {
    pub finished: bool,
    pub request_id: u32,
    pub response_type: u32,
    pub params_json: *mut c_char,
}


fn post_object( obj: *const DartResponse) -> bool {
    unsafe {
        if let Some(func) = POST_C_OBJECT {
            let boxed_msg = Box::new(dart_types::DartCObject {
                ty: dart_types::DartCObjectType::DartInt64,
                value: dart_types::DartCObjectValue { as_int64: obj as i64 },
            });
            let ptr = Box::into_raw(boxed_msg);
            // Send the message
            let result = func(NATIVE_PORT, ptr);
            // free the object
            let boxed_obj = Box::from_raw(ptr);
            drop(boxed_obj);
            result
        } else {
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dart_response_free(ptr: *mut libc::c_void)
{
    let response = Box::from_raw(ptr as *mut DartResponse);
    drop(response);
}

#[no_mangle]
pub unsafe extern "C" fn dart_string_free(ptr: *mut c_char) {
    let s = CString::from_raw(ptr);
    drop(s);
}

#[no_mangle]
pub unsafe extern "C" fn store_dart_post_cobject(
    ptr: dart_types::DartPostCObjectFnType,
) {
    POST_C_OBJECT = Some(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn init_sdk_for_ios() {
    println!("init sdk for ios")
}


mod test {
    use crate::init_sdk_for_ios;

    #[test]
    fn init_test() {
        unsafe { init_sdk_for_ios(); }
    }
}