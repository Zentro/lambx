#!/bin/bash

for i in {1..1000}; do
    `cargo run >> testOut.txt`;
done;