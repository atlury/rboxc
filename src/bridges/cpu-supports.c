/* Native compiler builtin bridge for the pinned x86-64 GNU wc profile. */
#include <string.h>
int rboxc_cpu_supports (char const *feature)
{
  if (strcmp (feature, "avx2") == 0) return __builtin_cpu_supports ("avx2");
  if (strcmp (feature, "avx512f") == 0) return __builtin_cpu_supports ("avx512f");
  if (strcmp (feature, "avx512bw") == 0) return __builtin_cpu_supports ("avx512bw");
  return 0;
}
