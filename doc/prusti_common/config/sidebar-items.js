initSidebarItems({"fn":[["allow_unreachable_unsupported_code","Encode unsupported code as `assert false`, so that we report error messages only for unsupported code that is actually reachable."],["assert_timeout","The assert timeout (in milliseconds) passed to Silicon."],["be_rustc","Should Prusti behave exactly like rustc?"],["cache_path","In which folder should we store the Verification cache"],["check_foldunfold_state","Generate additional, slow, checks for the foldunfold algorithm"],["check_overflows","Check binary operations for overflows"],["check_panics","Should we check absence of panics?"],["contracts_lib","Location of ‘libprusti_contracts*.rlib’"],["delete_basic_blocks","Replace the given basic blocks with `assume false`."],["disable_name_mangling","Disable mangling of generated Viper names."],["dump","Generate a dump of the settings"],["dump_borrowck_info","Should we dump borrowck info?"],["dump_debug_info","Should we dump debug files?"],["dump_debug_info_during_fold","Should we dump debug files for fold/unfold generation?"],["dump_path_ctxt_in_debug_info","Should we dump the branch context state in debug files?"],["dump_reborrowing_dag_in_debug_info","Should we dump the reborrowing DAGs in debug files?"],["dump_viper_program","Should we dump the Viper program?"],["enable_cache","Should Prusti ignore cached verification results."],["enable_ghost_constraints","Enables ghost constraint parsing and resolving."],["enable_manual_axiomatization","Enable manual axiomatization of pure functions."],["enable_purification_optimization","Enable purification optimization for impure functions."],["enable_verify_only_basic_block_path","Verify only the path given in `VERIFY_ONLY_BASIC_BLOCK_PATH`."],["encode_bitvectors","Enable (highly hacky) support for bitvectors."],["encode_unsigned_num_constraint","Encode (and check) that unsigned integers are non-negative."],["extra_jvm_args","Get extra JVM arguments"],["extra_verifier_args","Get extra arguments for the verifier"],["foldunfold_state_filter","The Viper backend that should be used for the verification"],["full_compilation","Continue the compilation and generate the binary after Prusti terminates"],["get_filtered_args","Return vector of arguments filtered out by prefix"],["hide_uuids","Should Prusti hide the UUIDs of expressions and specifications."],["ignore_regions","Should the dumped debug files not contain lifetime regions?"],["intern_names","Intern Viper identifiers to shorten them when possible."],["internal_errors_as_warnings","Report internal errors as warnings instead of errors. Used for testing."],["json_communication","If true, communication with the server will be encoded as json and not the default of bincode."],["log_dir","In which folder should we store log/dumps?"],["max_log_file_name_length","What is the longest allowed length of a log file name? If this is exceeded, the file name is truncated."],["no_verify","Skip the verification"],["no_verify_deps","Skip the verification of dependencies"],["only_memory_safety","Verify only the core proof."],["optimizations","Which optimizations should be enabled"],["print_collected_verification_items","Should Prusti print the items collected for verification."],["print_desugared_specs","Should Prusti print the AST with desugared specifications."],["print_hash","Should Prusti print VerificationRequest hashes."],["print_typeckd_specs","Should Prusti print the type-checked specifications."],["produce_counterexample","Should Prusti produce a counterexample."],["quiet","Should we hide user messages?"],["server_address","When set, Prusti will connect to this server and use it for its verification backend (i.e. the things using the JVM/Viper). Set to “MOCK” to run the server off-thread, effectively mocking connecting to a server without having to start it up separately. e.g. “127.0.0.1:2468”"],["server_max_concurrency","The maximum amount of verification requests the server will work on concurrently."],["server_max_stored_verifiers","The maximum amount of instantiated viper verifiers the server will keep around for reuse. If not set, this defaults to `SERVER_MAX_CONCURRENT_VERIFICATION_OPERATIONS`. It also doesn’t make much sense to set this to less than that, since then the server will likely have to keep creating new verifiers, reducing the performance gained from reuse. Note: This does not limit how many verification requests the server handles concurrently, only the size of what is essentially its verifier cache."],["simplify_encoding","Should we simplify the encoding before passing it to Viper?"],["skip_unsupported_features","Skip features that are unsupported or partially supported"],["unsafe_core_proof","Use the new core proof suitable for unsafe code."],["use_more_complete_exhale","Use the Silicon configuration option `--enableMoreCompleteExhale`."],["verification_deadline","Set the deadline in seconds within which Prusti should encode and verify the program."],["verify_only_basic_block_path","Verify only the single execution path goes through the given basic blocks."],["verify_only_preamble","Verify only the preamble: domains, functions, and predicates."],["viper_backend","The Viper backend that should be used for the verification"]],"mod":[["commandline",""]],"struct":[["Optimizations",""]]});