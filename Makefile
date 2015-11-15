_PHONY: format

SRCS = $(shell find . -type f -name '*.rs' ! -path './bgfx-sys/src/*')

format:
	@for f in $(SRCS); do                                                               \
		echo "Formatting $$f";                                                          \
		(sed -b 's/^\/\/\(#\[rustfmt_skip\]\)\s*$$/\1/g' "$$f" > "$$f.fmt");            \
		rustfmt --write-mode=overwrite "$$f.fmt" | grep -v "Using rustfmt config file"; \
		(sed -b 's/^\(#\[rustfmt_skip\]\)\s*$$/\/\/\1/g' "$$f.fmt" > "$$f");            \
		rm "$$f.fmt";                                                                   \
	done
