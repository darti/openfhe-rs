#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct CKKSParams {
    inner: pParamsCKKS,
}

impl Default for CKKSParams {
    fn default() -> Self {
        Self {
            inner: unsafe { params_new() },
        }
    }
}

impl CKKSParams {
    pub fn set_multiply_depth(&mut self, depth: u32) {
        unsafe { params_set_multiplication_depth(self.inner, depth) }
    }

    pub fn set_scale_mod_size(&mut self, size: u32) {
        unsafe { params_set_scaling_mod_size(self.inner, size) }
    }

    pub fn set_batch_size(&mut self, size: u32) {
        unsafe { params_set_batch_size(self.inner, size) }
    }
}

pub struct CryptoContext {
    inner: pCryptoContext,
}

impl CryptoContext {
    pub fn new(params: &mut CKKSParams) -> Self {
        Self {
            inner: unsafe { crypto_context_new(&mut params.inner) },
        }
    }
}
