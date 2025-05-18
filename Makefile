BIN=/usr/bin
NAME=ASCII
OUT_BIN=target/release/$(NAME)

.PHONY: all check-cargo build install uninstall

all: install

check-cargo:
	@command -v cargo >/dev/null 2>&1 || { \
		echo >&2 "Error: 'cargo' is not installed. Install it from https://rust-lang.org"; \
		exit 1; \
	}

build: check-cargo
	cargo build --release

install: 
	install -Dm755 $(OUT_BIN) $(DESTDIR)$(BIN)/$(NAME)
	@echo "Installed to $(BIN)/$(NAME)"

uninstall:
	rm -f $(BIN)/$(NAME)
	@echo "Removed $(BIN)/$(NAME)"
