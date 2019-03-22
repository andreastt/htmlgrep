m1 = doc/css.1.pod
man1 = $(m1:.pod=)
html1 = $(m1:.pod=.html)

man: $(man1) $(html1)

man-test: $(m1)
	podchecker $<

man-clean:
	rm -f doc/*.1
	rm -f doc/*.html

man-install:
	@echo

man-uninstall:
	@echo

.PHONY: man man-test man-clean man-install man-uninstall

%.1: %.1.pod
	pod2man -utf8 -c'htmlgrep suite' -ncss -r'$(shell git describe --tags)' $< >$@

%.1.html: %.1.pod
	pod2html $< >$@
