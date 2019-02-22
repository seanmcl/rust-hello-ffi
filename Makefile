
all:
	cargo build
	gcc -o ffi -L target/debug -lstringtools ffi.c
	./ffi

clean:
	cargo clean
	rm -rf ffi Cargo.lock
