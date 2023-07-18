all: libc_call_rust.a
	cd $(PWD)/src/clib;gcc main.c -o main libc_call_rust.a
	cd $(PWD)/src/clib;./main
libc_call_rust.a:
	cargo build
	cd $(PWD)/target/debug;cp libc_call_rust.a ../../src/clib
clean:
	cargo clean
	rm -rf $(PWD)/src/clib/*.a
	rm -rf $(PWD)/src/clib/main
