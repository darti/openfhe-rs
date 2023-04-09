// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern crate openmp_sys;

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
