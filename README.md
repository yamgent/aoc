# aoc

Advent of Code runner

## Assumptions

We assume that:

- Your code is written in Python 3.
- Program filename is `<part>.py` (e.g. part 1: `1.py`)
- Live input is `<part>.txt` (e.g. part 1: `1.txt`)
- Live output is `<part>.out.txt` (e.g. part 1: `1.out.txt`)
- All test cases input must be of the format `t<part>{suffix}.txt`, where suffix can be optional (e.g. part 1 can have `t1.txt` or `t1.1.txt`)
- All test cases output must be of the format `t<part>{suffix}.out.txt`, where suffix can be optional (e.g. part 1 can have `t1.out.txt` or `t1.1.out.txt`)

Note that live input/output means your actual inputs/outputs for the problem.
You can also have as many test cases as you want, but we only handle 1 live input.

## Installation

No official crate release for now; this must be compiled and installed manually:

```bash
git clone https://github.com/yamgent/aoc.git
cd aoc
cargo install --path .
```

## How to Use

All examples below assume you want to run part 1.

### Run code against live input

```bash
aoc run 1
```

### Run code against another input

For example, to run for `my_input.txt`:

```bash
aoc run 1 my_input
```

### Write live output

```bash
aoc write 1
```

### Write output for another input

For example, to run for `my_input.txt` and get `my_input.out.txt`:

```bash
aoc write 1 my_input
```

### Run all test cases

Note that both pairs of input and output must exist (pairs are determined
by `t<part>{suffix}`, so `t1.1.txt` pairs with `t1.1.out.txt`), otherwise
the test case will also fail.

Optionally, if live input/output pair exists, this pair will also be tested.

```bash
aoc test 1
```

### Run for a particular test case

For example, to test input `my_test.txt` and expected output `my_test.out.txt`:

```bash
aoc test 1 my_test
```

### Run test cases with diff

In case you want to view the output, you can use the `--diff` option:

```bash
aoc test 1 --diff
```

## Development

We use Rust + cargo for main development, Python 3 for system testing.

For debugging/building:

```bash
cargo run
cargo build
```

For system testing (run in main directory):

```bash
python3 testing/system_test.py
```
