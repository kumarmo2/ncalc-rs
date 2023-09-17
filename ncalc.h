#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CResult {
  const int64_t *int_result;
  const double *float_result;
  const uint8_t *bool_result;
  const uint8_t *string_result;
  const uint8_t *error;
} CResult;

void free_cresult(struct CResult result);

struct CResult evaluate(const char *formula);
