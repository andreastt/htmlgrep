POD2MAN = pod2man -utf8 -c'htmlgrep suite' -ncss -r'$(shell git describe --tags)' -qnone

m1 = doc/css.1.pod
man1 = $(m1:.pod=)
html1 = $(m1:.pod=.html)

man: $(man1) $(html1)

man-test: $(m1)
	podchecker $<

man-clean:
	rm -f doc/*.1

man-install:
	@echo

man-uninstall:
	@echo

.PHONY: man man-test man-clean man-install man-uninstall

%.1: %.1.pod
	$(POD2MAN) $< >$@

%.1.html: %.1.pod
	pod2html $< >$@
	rm -f pod2htmd.tmp
