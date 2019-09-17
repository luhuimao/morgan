/**
 * @brief test program that generates BPF PC relative call instructions
 */

#include <morgan_interface.h>

void __attribute__ ((noinline)) helper() {
  sol_log(__func__);
}

extern bool entrypoint(const uint8_t *input) {
  sol_log(__func__);
  helper();
  return true;
}

