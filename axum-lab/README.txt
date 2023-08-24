cargo install cargo-watch
sudo apt install libssl-dev pkg-config

# Run main bin continuously
cargo watch -q -c -w src/ -x run

# Run tests continuously
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

