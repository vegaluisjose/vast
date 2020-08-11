V05_DIR = $(abspath .)/regression/v05
V17_DIR = $(abspath .)/regression/v17

.PHONY: test
test:
	cargo fmt -- --check
	cargo clippy --tests
	cargo test --tests

.PHONY: test-lint
test-lint: test-lint-v05 test-lint-v17

.PHONY: test-lint-v05
test-lint-v05:
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_empty.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_one_input.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_three_inputs.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_one_param.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_two_params.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/module_mix_params.v
	verilator --lint-only +1364-2005ext+v $(V05_DIR)/prim.v $(V05_DIR)/module_with_instances.v

.PHONY: test-lint-v17
test-lint-v17:
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_empty.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_one_input.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_four_inputs.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/prim.v $(V17_DIR)/module_with_instances.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_with_function.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_with_function_add_one.v
	verilator --lint-only +1800-2017ext+v $(V17_DIR)/module_with_always_comb.v
