_PHONY: format

SRCS = $(shell find . -type f -name '*.rs' ! -path './bgfx-sys/src/*')

format: $(SRCS)
	rustfmt --write-mode=overwrite $(SRCS)
