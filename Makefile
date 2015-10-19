_PHONY: format

SRCS = $(shell find . -type f -name '*.rs' ! -path './bgfx-sys/src/*')

format:
	@for f in $(SRCS); do                                                              \
		echo "Formatting $$f";                                                     \
		(sed 's/^\/\/\(#\[rustfmt_skip\]\)$$/\1/g' "$$f" > "$$f.fmt");             \
		rustfmt --write-mode=overwrite "$$f.fmt" | grep -v "Project config file:"; \
		(sed 's/^\(#\[rustfmt_skip\]\)$$/\/\/\1/g' "$$f.fmt" > "$$f");             \
		rm "$$f.fmt";                                                              \
	done
