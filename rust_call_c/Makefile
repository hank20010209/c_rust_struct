all: libcool.a
	cargo run

libcool.a: $(PWD)/src/clib/cool.c
	cd $(PWD)/src/clib; gcc -c cool.c
	cd $(PWD)/src/clib; ar -rcs libcool.a cool.o

clean:
	rm -rf $(PWD)/src/clib/*.o
	rm -rf $(PWD)/src/clib/*.a
	cargo clean