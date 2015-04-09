#!/usr/bin/bash

sudo apt-get install libcurl4-openssl-dev libelf-dev libdw-dev &&
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&
tar xzf master.tar.gz &&
mkdir kcov-master/build &&
cd kcov-master/build &&
cmake .. &&
make &&
sudo make install &&
cd ../.. &&