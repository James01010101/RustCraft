
# this is the most recent version ive "shipped"
# so increase by 1 to "ship" a new version
shippingVersion := "0.1.0"


# build all
build: clear
	cargo build --release
	

# run the game with release settings
run: clear build
	cargo run --release 
# > testOutput.txt


# cleans world before running
cleanworldrun: cleanworld run


# test crates
test: clear
	cargo test

#clean
clean: 
	cargo clean


cleanworld:
	rm -rf assets/data/Worlds/*



# make a compresses shipping version of the game to be saved so i can see my progress
ship:
	clear

	@# build the shipping version of the code
	cargo clean
	cargo build --release --features "shipping"

	@# delete if it exists and create the shipping folder
	@if [ -d "shipping/$(shippingVersion)" ]; then rm -rf shipping/$(shippingVersion); fi
	@mkdir -p shipping/$(shippingVersion)

	@# delete the zip folder if it exists
	rm -f shipping/$(shippingVersion).7z

	@# copy the files to the shipping folder
	cp target/release/RustCraft.exe shipping/$(shippingVersion)/
	cp -r assets shipping/$(shippingVersion)/

	@# remove the World folder so no world get copied across
	rm -rf shipping/$(shippingVersion)/assets/data/Worlds/

	@# compress the shipping folder (cd into the shipping directory first)
	cd shipping && 7z a -t7z $(shippingVersion).7z $(shippingVersion)/

	@# delete the shipping folder
	rm -rf shipping/$(shippingVersion)

clear: 
	clear