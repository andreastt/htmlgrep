A2X = XML_CATALOG_FILES=/usr/local/etc/xml/catalog a2x --verbose

m1 = doc/css.1.txt
man1 = $(m1:.txt=)

%.1: %.1.txt
	$(A2X) -f manpage $<

man: $(man1)

clean-man:
	rm -f doc/*.1
	rm -f doc/*.xml

install-man:
	@echo

uninstall-man:
	@echo

.PHONY: man clean-man install-man uninstall-man
