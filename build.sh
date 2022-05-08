#!/bin/bash

cargo build --release
cp ./target/release/lines_counter ./res
cp ./settings.json ./res