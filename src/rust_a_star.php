<?php
echo "attempting to load rust_a_star..\n";
if (!extension_loaded("rust_a_star")) {
    dl('librust_a_star.dylib');
}

echo "loaded!\n";

confirm_rust_a_star_compiled();

echo "called!\n";