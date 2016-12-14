## Valgrind
### with extension
valgrind --leak-check=full ~/proj_/php/php-out/bin/php src/rust_a_star.php
==86419== Memcheck, a memory error detector
==86419== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==86419== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==86419== Command: /Users/tim/proj_/php/php-out/bin/php src/rust_a_star.php
==86419==
--86419-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option
--86419-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 2 times)
--86419-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 4 times)
--86419-- run: /usr/bin/dsymutil "/Users/tim/proj_/rust/hello_from_rust/target/debug/librust_a_star.dylib"
warning: (x86_64) /Users/tim/proj_/rust/hello_from_rust/target/debug/deps/rust_a_star.0.o unable to open object file: No such file or directory
warning: no debug symbols in executable (-arch x86_64)
==86419== Warning: ignored attempt to set SIGUSR2 handler in sigaction();
==86419==          the SIGUSR2 signal is used internally by Valgrind
Could not startup.
==86419==
==86419== HEAP SUMMARY:
==86419==     in use at exit: 39,771 bytes in 232 blocks
==86419==   total heap usage: 12,532 allocs, 12,300 frees, 1,756,703 bytes allocated
==86419==
==86419== 2 bytes in 1 blocks are definitely lost in loss record 1 of 107
==86419==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x1020301BF: ???
==86419==    by 0x102030130: ???
==86419==    by 0x102030355: ???
==86419==    by 0x1020302FD: ???
==86419==    by 0x10204AB6B: ???
==86419==    by 0x10202FDB7: ???
==86419==    by 0x102030292: ???
==86419==    by 0x10203024C: ???
==86419==    by 0x1003CA679: php_load_extension (dl.c:162)
==86419==    by 0x10048B16E: php_load_php_extension_cb (php_ini.c:344)
==86419==    by 0x100510316: zend_llist_apply (zend_llist.c:184)
==86419==
==86419== 40 bytes in 1 blocks are definitely lost in loss record 47 of 107
==86419==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x1014AF568: _collecting_in_critical() (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A9D4C: lookUpImpOrForward (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A9CCE: lookUpImpOrForward (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x10145ED68: -[OS_xpc_object _xref_dispose] (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145FCC9: xpc_pipe_routine (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145F9DD: _xpc_interface_routine (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145F5C3: bootstrap_look_up3 (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145F4F8: bootstrap_look_up2 (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x1013E6907: ___notify_lib_init_block_invoke (in /usr/lib/system/libsystem_notify.dylib)
==86419==    by 0x1010AA40A: _dispatch_client_callout (in /usr/lib/system/libdispatch.dylib)
==86419==    by 0x1010AA302: dispatch_once_f (in /usr/lib/system/libdispatch.dylib)
==86419==
==86419== 64 bytes in 1 blocks are definitely lost in loss record 60 of 107
==86419==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x100F0A773: xmlNewMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100F09C72: xmlInitGlobals (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100EB4B04: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86419==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86419==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86419==    by 0x1005307BF: zend_startup_module_zval (in /Users/tim/proj_/php/php-out/bin/php)
==86419==    by 0x100541D7C: zend_hash_apply (zend_hash.c:1505)
==86419==    by 0x1005305E6: zend_startup_modules (zend_API.c:1969)
==86419==    by 0x10047ABFC: php_module_startup (main.c:2254)
==86419==    by 0x100647C3A: php_cli_startup (php_cli.c:428)
==86419==
==86419== 64 bytes in 1 blocks are definitely lost in loss record 61 of 107
==86419==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x100F0A773: xmlNewMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100ECB6BB: xmlInitMemory (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100EB4B2C: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86419==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86419==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86419==    by 0x1005307BF: zend_startup_module_zval (in /Users/tim/proj_/php/php-out/bin/php)
==86419==    by 0x100541D7C: zend_hash_apply (zend_hash.c:1505)
==86419==    by 0x1005305E6: zend_startup_modules (zend_API.c:1969)
==86419==    by 0x10047ABFC: php_module_startup (main.c:2254)
==86419==    by 0x100647C3A: php_cli_startup (php_cli.c:428)
==86419==
==86419== 104 (56 direct, 48 indirect) bytes in 1 blocks are definitely lost in loss record 71 of 107
==86419==    at 0x100D615B9: calloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x1014A95BA: call_load_methods (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A951F: call_load_methods (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A91D8: prepare_load_methods (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A8D07: NXMapRemove (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A3590: objc_msgSend (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x10145FCC9: xpc_pipe_routine (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145F9DD: _xpc_interface_routine (in /usr/lib/system/libxpc.dylib)
==86419==    by 0x10145F5C3: bootstrap_look_up3 (in /usr/lib/system/libxpc.dylib)
==86419==
==86419== 128 bytes in 1 blocks are definitely lost in loss record 78 of 107
==86419==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x100F0A7EE: xmlNewRMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100F55ACF: __xmlInitializeDict (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x101404BF5: __pthread_once_handler (in /usr/lib/system/libsystem_pthread.dylib)
==86419==    by 0x1013F0FC3: _os_once (in /usr/lib/system/libsystem_platform.dylib)
==86419==    by 0x101404B94: pthread_once (in /usr/lib/system/libsystem_pthread.dylib)
==86419==    by 0x100F0AA3C: xmlIsMainThread (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100F0A2BB: __xmlGenericError (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x100EB4B09: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86419==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86419==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86419==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86419==
==86419== 2,064 bytes in 1 blocks are possibly lost in loss record 101 of 107
==86419==    at 0x100D6117C: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86419==    by 0x1014B3EFD: _objc_copyClassNamesForImage (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A7182: protocols() (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A7093: readClass(objc_class*, bool, bool) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A4C13: gc_init (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014AC24E: objc_initializeClassPair_internal(objc_class*, char const*, objc_class*, objc_class*) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014B9132: layout_string_create (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A783C: realizeClass(objc_class*) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A7300: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86419==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86419==
==86419== LEAK SUMMARY:
==86419==    definitely lost: 354 bytes in 6 blocks
==86419==    indirectly lost: 48 bytes in 2 blocks
==86419==      possibly lost: 2,064 bytes in 1 blocks
==86419==    still reachable: 17,080 bytes in 38 blocks
==86419==         suppressed: 20,225 bytes in 185 blocks
==86419== Reachable blocks (those to which a pointer was found) are not shown.
==86419== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==86419==
==86419== For counts of detected and suppressed errors, rerun with: -v
==86419== ERROR SUMMARY: 7 errors from 7 contexts (suppressed: 20 from 20)
make: *** [valgrind] Error 1

### without extension

valgrind --leak-check=full ~/proj_/php/php-out/bin/php src/rust_a_star.php
==86521== Memcheck, a memory error detector
==86521== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==86521== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==86521== Command: /Users/tim/proj_/php/php-out/bin/php src/rust_a_star.php
==86521==
--86521-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option
--86521-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 2 times)
--86521-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 4 times)
==86521== Warning: ignored attempt to set SIGUSR2 handler in sigaction();
==86521==          the SIGUSR2 signal is used internally by Valgrind
Could not startup.
==86521==
==86521== HEAP SUMMARY:
==86521==     in use at exit: 39,433 bytes in 229 blocks
==86521==   total heap usage: 12,517 allocs, 12,288 frees, 1,755,502 bytes allocated
==86521==
==86521== 40 bytes in 1 blocks are definitely lost in loss record 46 of 104
==86521==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x1014AF568: _collecting_in_critical() (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A9D4C: lookUpImpOrForward (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A9CCE: lookUpImpOrForward (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x10145ED68: -[OS_xpc_object _xref_dispose] (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145FCC9: xpc_pipe_routine (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145F9DD: _xpc_interface_routine (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145F5C3: bootstrap_look_up3 (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145F4F8: bootstrap_look_up2 (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x1013E6907: ___notify_lib_init_block_invoke (in /usr/lib/system/libsystem_notify.dylib)
==86521==    by 0x1010AA40A: _dispatch_client_callout (in /usr/lib/system/libdispatch.dylib)
==86521==    by 0x1010AA302: dispatch_once_f (in /usr/lib/system/libdispatch.dylib)
==86521==
==86521== 64 bytes in 1 blocks are definitely lost in loss record 59 of 104
==86521==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x100F0A773: xmlNewMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100F09C72: xmlInitGlobals (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100EB4B04: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86521==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86521==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86521==    by 0x1005307BF: zend_startup_module_zval (in /Users/tim/proj_/php/php-out/bin/php)
==86521==    by 0x100541D7C: zend_hash_apply (zend_hash.c:1505)
==86521==    by 0x1005305E6: zend_startup_modules (zend_API.c:1969)
==86521==    by 0x10047ABFC: php_module_startup (main.c:2254)
==86521==    by 0x100647C3A: php_cli_startup (php_cli.c:428)
==86521==
==86521== 64 bytes in 1 blocks are definitely lost in loss record 60 of 104
==86521==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x100F0A773: xmlNewMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100ECB6BB: xmlInitMemory (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100EB4B2C: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86521==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86521==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86521==    by 0x1005307BF: zend_startup_module_zval (in /Users/tim/proj_/php/php-out/bin/php)
==86521==    by 0x100541D7C: zend_hash_apply (zend_hash.c:1505)
==86521==    by 0x1005305E6: zend_startup_modules (zend_API.c:1969)
==86521==    by 0x10047ABFC: php_module_startup (main.c:2254)
==86521==    by 0x100647C3A: php_cli_startup (php_cli.c:428)
==86521==
==86521== 104 (56 direct, 48 indirect) bytes in 1 blocks are definitely lost in loss record 70 of 104
==86521==    at 0x100D615B9: calloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x1014A95BA: call_load_methods (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A951F: call_load_methods (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A91D8: prepare_load_methods (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A9155: class_createInstance (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A8D07: NXMapRemove (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A3590: objc_msgSend (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x10145FCC9: xpc_pipe_routine (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145F9DD: _xpc_interface_routine (in /usr/lib/system/libxpc.dylib)
==86521==    by 0x10145F5C3: bootstrap_look_up3 (in /usr/lib/system/libxpc.dylib)
==86521==
==86521== 128 bytes in 1 blocks are definitely lost in loss record 76 of 104
==86521==    at 0x100D60EBB: malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x100F0A7EE: xmlNewRMutex (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100F55ACF: __xmlInitializeDict (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x101404BF5: __pthread_once_handler (in /usr/lib/system/libsystem_pthread.dylib)
==86521==    by 0x1013F0FC3: _os_once (in /usr/lib/system/libsystem_platform.dylib)
==86521==    by 0x101404B94: pthread_once (in /usr/lib/system/libsystem_pthread.dylib)
==86521==    by 0x100F0AA3C: xmlIsMainThread (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100F0A2BB: __xmlGenericError (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x100EB4B09: xmlInitParser (in /usr/local/Cellar/libxml2/2.9.4/lib/libxml2.2.dylib)
==86521==    by 0x10005EC45: php_libxml_initialize (libxml.c:755)
==86521==    by 0x10005DB95: zm_startup_libxml (libxml.c:794)
==86521==    by 0x10052FEBC: zend_startup_module_ex (zend_API.c:1843)
==86521==
==86521== 2,064 bytes in 1 blocks are possibly lost in loss record 98 of 104
==86521==    at 0x100D6117C: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.11.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==86521==    by 0x1014B3EFD: _objc_copyClassNamesForImage (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A7182: protocols() (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A7093: readClass(objc_class*, bool, bool) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A4C13: gc_init (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014AC24E: objc_initializeClassPair_internal(objc_class*, char const*, objc_class*, objc_class*) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014B9132: layout_string_create (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A783C: realizeClass(objc_class*) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A7300: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86521==    by 0x1014A72E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==86521==
==86521== LEAK SUMMARY:
==86521==    definitely lost: 352 bytes in 5 blocks
==86521==    indirectly lost: 48 bytes in 2 blocks
==86521==      possibly lost: 2,064 bytes in 1 blocks
==86521==    still reachable: 16,744 bytes in 36 blocks
==86521==         suppressed: 20,225 bytes in 185 blocks
==86521== Reachable blocks (those to which a pointer was found) are not shown.
==86521== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==86521==
==86521== For counts of detected and suppressed errors, rerun with: -v
==86521== ERROR SUMMARY: 6 errors from 6 contexts (suppressed: 20 from 20)
make: *** [valgrind] Error 1