#include "parameters.hpp"

typedef struct
{
    void *context;
} pCryptoContext;

extern "C"
{
    pCryptoContext crypto_context_new(pParamsCKKS *params);
}