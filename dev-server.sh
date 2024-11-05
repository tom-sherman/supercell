#!/bin/sh

export HTTP_PORT=4050
export EXTERNAL_BASE=feeds.smokesignal.events
export DATABASE_URL=sqlite://development.db
export JETSTREAM_HOSTNAME=jetstream1.us-east.bsky.network
export ZSTD_DICTIONARY=$(pwd)/jetstream_zstd_dictionary
export CONSUMER_TASK_ENABLE=true
export FEEDS=$(pwd)/config.yml

touch development.db
sqlx migrate run --database-url sqlite://development.db

RUST_BACKTRACE=1 RUST_LOG=debug RUST_LIB_BACKTRACE=1 cargo run --bin supercell

