

# build all
build: clear
	cargo build --release
	

# run the game with release settings
run: clear build
	cargo run --release > testOutput.txt

# test crates
test: clear
	cargo test

#clean
clean: 
	cargo clean


clear: 
	clear