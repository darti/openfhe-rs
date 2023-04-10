typedef void *pParamsCKKS;

extern "C"
{
    pParamsCKKS params_new();

    unsigned int params_get_multiplication_depth(pParamsCKKS self);

    void params_set_multiplication_depth(pParamsCKKS params, unsigned int depth);

    unsigned int params_get_scaling_mod_size(pParamsCKKS self);

    void params_set_scaling_mod_size(pParamsCKKS params, unsigned int scale_mod_size);

    void params_set_batch_size(pParamsCKKS params, unsigned int batch_size);

    unsigned int params_get_batch_size(pParamsCKKS params);
}