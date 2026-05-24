[targets.program]
depends = ["src/main.rs", "src/lib.rs"]
command = "rustc src/main.rs --edition 2021 -o program"

[targets.test]
depends = ["src/lib.rs", "tests/test.rs"]
command = "rustc --test tests/test.rs --edition 2021 -o test_runner && ./test_runner"

[targets.run]
depends = ["program"]
command = "./program"

[targets.clean]
depends = []
command = "rm -f program test_runner"
