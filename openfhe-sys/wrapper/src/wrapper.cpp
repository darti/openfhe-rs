#include "openfhe.h"
#include "wrapper.hpp"

pParamsCKKS *params_new()
{
    return new pParamsCKKS;
}

// void params_set_mult_depth(pParamsCKKS *params, int depth)
// {
//     params->SetMultiplicativeDepth(depth);
// }