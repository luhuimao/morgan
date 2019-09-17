/**
 * @brief Example C-based BPF program that moves funds from one account to
 * another
 */

#include <morgan_interface.h>

/**
 * Number of SolKeyedAccount expected. The program should bail if an
 * unexpected number of accounts are passed to the program's entrypoint
 */
#define NUM_KA 3

extern bool entrypoint(const uint8_t *input) {
  SolKeyedAccount ka[NUM_KA];
  SolParameters params = (SolParameters) { .ka = ka };

  if (!sol_deserialize(input, &params, SOL_ARRAY_SIZE(ka))) {
    return false;
  }

  if (!params.ka[0].is_signer) {
    sol_log("Transaction not signed by key 0");
    return false;
  }

  int64_t difs = *(int64_t *)params.data;
  if (*params.ka[0].difs >= difs) {
    *params.ka[0].difs -= difs;
    *params.ka[2].difs += difs;
    // sol_log_64(0, 0, *ka[0].difs, *ka[2].difs, difs);
  } else {
    // sol_log_64(0, 0, 0xFF, *ka[0].difs, difs);
  }
  return true;
}
