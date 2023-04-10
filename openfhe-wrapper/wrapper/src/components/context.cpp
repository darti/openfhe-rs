#include "context.hpp"
#include "openfhe.h"
#include "parameters.hpp"

using namespace lbcrypto;

pCryptoContext crypto_context_new(pParamsCKKS params)
{
    auto p = reinterpret_cast<CCParams<CryptoContextCKKSRNS> *>(params);

    CryptoContext<DCRTPoly> cc = GenCryptoContext(*p);

    return cc.get();
}
