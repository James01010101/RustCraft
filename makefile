

# build all
build_all: clear build_engine build_game

build_engine:
	cd GameEngine && cargo build --release

build_game:
	cd RustCraft && cargo build --release


# run the game with release settings
run_game: clear build_game
	cd RustCraft && cargo run --release


# test crates
test_game:
	clear
	cd RustCraft && cargo test

test_engine:
	clear
	cd GameEngine && cargo test


# clean the crates
clean: clear clean_engine clean_game

clean_engine:
	cd GameEngine && cargo clean

clean_game:
	cd RustCraft && cargo clean


clear: 
	clear