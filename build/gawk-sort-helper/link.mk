.PHONY: rboxc-print
rboxc-print:
	@printf '%s\n' $(CCLD) $(AM_CFLAGS) $(CFLAGS) $(AM_LDFLAGS) $(LDFLAGS) $(src_coreutils_OBJECTS) $(src_coreutils_LDADD) $(LIBS)
