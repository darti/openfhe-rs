#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct CKKSParams {
    inner: *mut pParamsCKKS,
}

impl Default for CKKSParams {
    fn default() -> Self {
        Self {
            inner: unsafe { params_new() },
        }
    }
}
