use openfhe_wrapper::lbcrypto_CCParams;

#[test]
fn create_params() {
    let _params = CKKSParams::default();
}

#[test]
fn set_params() {
    let mut params = lbcrypto_CCParams {};

    params.set_multiply_depth(1);
    assert!(params.get_multiply_depth() == 1);

    params.set_scale_mod_size(50);
    assert!(params.get_scale_mod_size() == 50);

    params.set_batch_size(8);
    assert!(params.get_batch_size() == 8);
}

#[test]
fn create_context() {
    let mut params = CKKSParams::default();

    params.set_multiply_depth(1);
    params.set_scale_mod_size(50);
    params.set_batch_size(8);

    let _cc = openfhe_wrapper::CryptoContext::new(&params);
}
