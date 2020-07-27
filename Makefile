clean:
	ssh pi@monitelec.home "cd teleinfo-reader && rm -rfv target"

build-rpi-release:
	ssh pi@monitelec.home "cd teleinfo-reader && /home/pi/.cargo/bin/cargo build --release"

build-rpi-debug:
	ssh pi@monitelec.home "cd teleinfo-reader && /home/pi/.cargo/bin/cargo build"

