m1 = doc/css.1.txt
man1 = $(m1:.txt=)

%.1: %.1.txt
	a2x -f manpage $<

man: $(man1)

clean-man:
	rm -f $(man1)

install-man:
	@echo

uninstall-man:
	@echo

.PHONY: man clean-man install-man uninstall-man
