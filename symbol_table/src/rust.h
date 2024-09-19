#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "rsm.h"
#include "symbol.h"

int16_t TMP_Locate(var_u var, const table_struct *table);

int16_t TMP_Create(var_u var, table_struct *table);

void TMP_Free(var_u var, table_struct *table);
