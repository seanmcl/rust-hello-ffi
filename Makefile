
all:
	cargo build
	gcc -o ffi -L target/debug -lstringtools ffi.c
	./ffi
	python ffi.py

clean:
	cargo clean
	rm -rf ffi Cargo.lock
