#!/bin/bash

set -o

sudo apt-get update
sudo apt-get install gcc -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
