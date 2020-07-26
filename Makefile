clean:
	rm -rf target

build-rpi:
	cross build --target armv7-unknown-linux-musleabihf --release

