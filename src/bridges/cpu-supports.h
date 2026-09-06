/* Preserve GNU cpu_supports' runtime and GLIBC_TUNABLES selection. Only
   the compiler builtin itself is delegated to a native C helper. */
int rboxc_cpu_supports (char const *feature);
#define __builtin_cpu_supports(feature) rboxc_cpu_supports(feature)
