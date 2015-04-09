#!/usr/bin/bash

sudo apt-get install libdwarf-dev libdw-dev libelf-dev elfutils libcurl4-openssl-dev cmake binutils-dev
wget https://github.com/SimonKagstrom/kcov/archive/v26.tar.gz
tar xzf v26.tar.gz
mkdir kcov-26/build
cd kcov-26/build
cmake -DCMAKE_BUILD_TYPE=Release ..
make
sudo make install
cd ../..
