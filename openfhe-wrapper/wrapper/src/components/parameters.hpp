typedef void *pParamsCKKS;

extern "C"
{
    pParamsCKKS *params_new();

    void params_set_multiplication_depth(pParamsCKKS *params, unsigned int depth);

    void params_set_scaling_mod_size(pParamsCKKS *params, unsigned int scale_mod_size);

    void params_set_batch_size(pParamsCKKS *params, unsigned int batch_size);
}