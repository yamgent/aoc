# SPDX-License-Identifier: MIT
import os
import sys
import subprocess

AOC_PATH = '../../target/debug/aoc'
TESTING_PATH = './testing/'


def get_test_directory_path(directory):
    return TESTING_PATH + directory + '/'


def clean(dir_path, files):
    for file in files:
        if os.path.isfile(dir_path + file):
            os.remove(dir_path + file)


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
    process = subprocess.run([AOC_PATH] + aoc_args, check=True, capture_output=True, cwd=get_test_directory_path(directory))
    assert_eq(process.stdout.decode('utf-8'), expected, test_name + ' test FAILED')


def do_writing_test_case(test_name, directory, files_to_clean, aoc_args, expected_filename, expected):
    dir_path = get_test_directory_path(directory)
    expected_path = dir_path + expected_filename
    print('Test', test_name)

    clean(dir_path, files_to_clean)

    subprocess.run([AOC_PATH] + aoc_args, check=True, cwd=dir_path)
    if expected is None:
        assert_eq('exist' if os.path.isfile(expected_path) else None, None, test_name + ' test FAILED')
    else:
        actual = open(expected_path, 'r').read() if os.path.isfile(expected_path) else ''
        assert_eq(actual, expected, test_name + ' test FAILED')

    clean(dir_path, files_to_clean)


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

    # write live should write 
    do_writing_test_case('Write Live', 'write', ['1.out.txt', '1.other.out.txt'], ['write', '1'], '1.out.txt', '\n'.join([
        'The quick',
        'brown fox',
        'lazy dog.',
        ''
    ]))
    do_writing_test_case('Write Live No Touch Others', 'write', ['1.out.txt', '1.other.out.txt'], ['write', '1'], '1.other.out.txt', None)

    # write custom should write
    do_writing_test_case('Write Others', 'write', ['1.out.txt', '1.other.out.txt'], ['write', '1', '1.other'], '1.other.out.txt', '\n'.join([
        'This is alternative',
        'input.',
        ''
    ]))
    do_writing_test_case('Write Others No Touch Live', 'write', ['1.out.txt', '1.other.out.txt'], ['write', '1', '1.other'], '1.out.txt', None)


main()
