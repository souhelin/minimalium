#ifndef MINIMALIUM_H
#define MINIMALIUM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MinimaliumEngine MinimaliumEngine;

uint32_t minimalium_engine_abi_version(void);
MinimaliumEngine *minimalium_engine_create(void);
void minimalium_engine_destroy(MinimaliumEngine *engine);

/* Returns 0 on success; nonzero values indicate invalid arguments or target.
 * This validates and records a URL only. It does not perform network I/O. */
int32_t minimalium_engine_navigate(MinimaliumEngine *engine, const char *target);
int32_t minimalium_engine_status_ok(void);

#ifdef __cplusplus
}
#endif

#endif
