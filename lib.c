#include <stdint.h>

/** A function that must be present in the API. */
__attribute__((visibility("default"))) int32_t foo() { return 4; }

/** A function that may or may not be present in the API. */
// __attribute__((visibility("default"))) int32_t bar() { return 10; }
