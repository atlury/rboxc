/* Standalone native GNU test oracle, not an applet port. */
extern int single_binary_main_printenv(int, char **);
int main(int argc, char **argv) { return single_binary_main_printenv(argc, argv); }
