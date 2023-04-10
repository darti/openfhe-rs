#include "parameters.hpp"

typedef void *pCryptoContext;

extern "C"
{
    pCryptoContext crypto_context_new(pParamsCKKS params);
}