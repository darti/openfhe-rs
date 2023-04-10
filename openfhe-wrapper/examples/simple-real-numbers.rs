use openfhe_wrapper::CKKSParams;

const MULT_DEPTH: u32 = 1;
const SCALE_MOD_SIZE: u32 = 50;

const BATCH_SIZE: u32 = 8;

fn main() {
    let mut parameters = CKKSParams::default();
    parameters.set_multiply_depth(MULT_DEPTH);
    parameters.set_scale_mod_size(SCALE_MOD_SIZE)
}
