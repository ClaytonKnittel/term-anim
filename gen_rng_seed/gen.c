#include <inttypes.h>
#include <stdio.h>

int main() {
  const char seed[] = "Eugenia";
  uint64_t seed_n = *((uint64_t*) seed);
  printf("Seed: %" PRIu64 "\n", seed_n);
  return 0;
}
