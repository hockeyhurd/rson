#!/bin/bash

TEST_DIR=$1

if [ -z "$1" ]; then
    TEST_DIR='tests'
fi

echo "TEST_DIR: " $TEST_DIR

function unitTests()
{
    #echo $PWD
    local testFiles=`find $TEST_DIR -type f -name "*.json" -print`
    echo "Tests to run: " $testFiles

    for test in $testFiles
    do
        echo "Starting test: " $test
        cargo run -- --input $test
        retVal=$?
        #echo 'Result: '$retVal
        echo "Test: " $test " completed with exit code: " $retVal

        if [ $retVal -ne 0 ]; then
            exit $retVal
        fi
    done
}

time unitTests

echo "Done"

