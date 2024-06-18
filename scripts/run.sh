#!/bin/bash

# Start Sass in watch mode without generating source maps
sass --no-source-map --style compressed --watch public/static/scss:public/static/css --quiet &

# Start cargo watch for Rust project
cargo watch -x run --ignore scripts,public/static/css

