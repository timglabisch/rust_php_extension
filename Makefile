build:
	cargo build
	cp target/debug/librust_a_star.dylib /Users/tim/proj_/php/php-src/../php-out/lib/php/extensions/debug-non-zts-20160303/librust_a_star.dylib

run: build
	~/proj_/php/php-out/bin/php src/rust_a_star.php

debug: build 
	gdb run -tui --args ~/proj_/php/php-out/bin/php src/rust_a_star.php

valgrind: 
	valgrind --leak-check=full ~/proj_/php/php-out/bin/php src/rust_a_star.php