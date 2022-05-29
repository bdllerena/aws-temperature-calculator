#!/bin/bash

cd lambda
cargo build --release
(cd target/release && mkdir -p lambda && cp bootstrap.d lambda/)