#include "openfhe.h"
#include "parameters.hpp"

using namespace lbcrypto;

pParamsCKKS params_new()
{
    return new CCParams<CryptoContextCKKSRNS>();
}

unsigned int params_get_multiplication_depth(pParamsCKKS self)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    return p->GetMultiplicativeDepth();
}

void params_set_multiplication_depth(pParamsCKKS self, unsigned int depth)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    p->SetMultiplicativeDepth(depth);
}

unsigned int params_get_scaling_mod_size(pParamsCKKS self)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    return p->GetScalingModSize();
}

void params_set_scaling_mod_size(pParamsCKKS self, unsigned int scale_mod_size)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    p->SetScalingModSize(scale_mod_size);
}

unsigned int params_get_batch_size(pParamsCKKS self)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    return p->GetBatchSize();
}

void params_set_batch_size(pParamsCKKS self, unsigned int batch_size)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(self);
    p->SetBatchSize(batch_size);
}
