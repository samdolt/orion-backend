#!/usr/bin/bash

mkdir target/kcov

export liborion=`find . -regextype posix-extended -regex '\./target/debug/orion\-[a-zA-Z0-9]{16,16}'`
export logger=`find . -regextype posix-extended -regex '\./target/debug/orion_logger\-[a-zA-Z0-9]{16,16}'`

echo "Found liborion test binary $liborion"
echo "Found logger test binary $logger"

kcov --coveralls-id=$TRAVIS_JOB_ID --exclude-pattern=/.cargo target/kcov/liborion $liborion;
kcov --coveralls-id=$TRAVIS_JOB_ID --exclude-pattern=/.cargo target/kcov/logger $logger;

