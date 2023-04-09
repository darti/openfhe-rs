typedef void *pParamsCKKS;

extern "C"
{
    pParamsCKKS *params_new();

    void params_set_multiplication_depth(pParamsCKKS *params, unsigned int depth);
}