#!/bin/bash

TEST_DIR=$1
BUILD_TYPE="--$2"

if [ -z "$1" ]; then
    TEST_DIR='tests'
fi

# Note: In cargo, there is no '--debug' but only '--release'.
# Since we default to running in debug, we simply set this to
# an empty string.
if [ -z "$2" ]; then
    BUILD_TYPE=''
elif [ "$2" == "debug" ]; then
    BUILD_TYPE=''
elif [ "$2" == "release" ]; then
    BUILD_TYPE='--release'
else
    echo "Un-supported build type."
    exit -1
fi

echo "TEST_DIR: " ${TEST_DIR}

function unitTests()
{
    #echo $PWD
    local testFiles=`find ${TEST_DIR} -type f -name "*.json" -print`
    echo "Tests to run: " ${testFiles}

    for test in ${testFiles}
    do
        echo "Starting test: " ${test}
        cargo run ${BUILD_TYPE} -- --input ${test}
        retVal=$?
        #echo 'Result: '$retVal
        echo "Test: " ${test} " completed with exit code: " ${retVal}

        if [ ${retVal} -ne 0 ]; then
            exit ${retVal}
        fi
    done
}

time unitTests

echo "Done"

