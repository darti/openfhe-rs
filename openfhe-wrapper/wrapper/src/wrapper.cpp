#include "openfhe.h"
#include "utils/inttypes.h"
#include "wrapper.hpp"

using namespace lbcrypto;

pParamsCKKS *params_new()
{
    return new pParamsCKKS();
}

void params_set_multiplication_depth(pParamsCKKS *self, unsigned int depth)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    p->SetMultiplicativeDepth(depth);
}