import sys
import subprocess

def assert_eq(actual, expected, message):
    if actual != expected:
        print(message)
        print('Expected:')
        print(expected)
        print('Actual:')
        print(actual)
        sys.exit(1)


def do_stdout_test_case(test_name, directory, aoc_args, expected):
    print('Test', test_name)
    process = subprocess.run(['../../target/debug/aoc'] + aoc_args, check=True, capture_output=True, cwd='./testing/' + directory)
    assert_eq(process.stdout.decode('utf-8'), expected, test_name + ' test FAILED')


def main():
    # build binary
    subprocess.run(['cargo', 'build'], check=True)

    ### TEST CASES
    # do run on live input
    do_stdout_test_case('Run Live', 'run', ['run', '1'], '\n'.join([
        'The quick',
        'brown fox',
        'lazy dog.',
        ''
    ]))

    # do run on custom input
    do_stdout_test_case('Run Other', 'run', ['run', '1', '1.other'], '\n'.join([
        'This is alternative',
        'input.',
        ''
    ]))

    # test all should succeed on success case
    do_stdout_test_case('Test All Should Pass', 'test_success', ['test', '1'], '\n'.join([
        '    t1.1.txt SUCCESS',
        '    t1.txt SUCCESS',
        '    1.txt SUCCESS',
        '',
        '3 success, 0 failure, 0 error',
        'All test cases passed.',
        ''
    ]))

    # test all should fail on failure cases
    do_stdout_test_case('Test All Should Warn', 'test_failure', ['test', '1'], '\n'.join([
        '(!) t1.noout1.txt OUTPUT-MISSING',
        '(!) t1.noout2.txt OUTPUT-MISSING',
        '(X) t1.wrong.txt FAILURE',
        '',
        '0 success, 1 failure, 2 error',
        'FAILED some test cases.',
        ''
    ]))

    # test one should succeed on success case
    do_stdout_test_case('Test Should Pass', 'test_success', ['test', '1', 't1.1'], '\n'.join([
        '    t1.1.txt SUCCESS',
        '',
        '1 success, 0 failure, 0 error',
        'All test cases passed.',
        ''
    ]))

    # test one should fail on failure cases
    do_stdout_test_case('Test Should Warn', 'test_failure', ['test', '1', 't1.wrong'], '\n'.join([
        '(X) t1.wrong.txt FAILURE',
        '',
        '0 success, 1 failure, 0 error',
        'FAILED some test cases.',
        ''
    ]))


main()
