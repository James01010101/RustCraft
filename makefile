

# build all
build: clear
	cargo build --release

# run the game with release settings
run: clear build
	cargo run --release

# test crates
test_game: clear
	cargo test

#clean
clean: 
	cargo clean


clear: 
	clear