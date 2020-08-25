# Teleinfo Reader

Rust program that read serial data from teleinformation flux.
Only work for the TEMPO subscription.
Send data into PostgreSQL database.


## Configuration

Create a `.env` file to configure the PostgreSQL connection with:
 - the variable `DATABASE_URL` (like `postgres://user:password@database-host:port/dbname`, see `diesel` documentation for more information),
 - the variable `TELEINFO_SERIAL` (path of serial device).

 
## CLI scripts

### Read teleinfomation data

The `read-teleinfo` executable read the serial data and send them into a PostgreSQL database.

Usage: `read-teleinfo [-v] [-c file] [device]`

The device path can be force. In this way, the device path set in the configuration file is ignored.
The configuration file path can be force to read a specific configuration. By default, the script read
`/etc/read-teleinfo.conf` and `.env` file.


### Data generator

The `data-generator` executable open a file socket and send fake serial data into it.


### Show stored data

The `show-data` executable read and show the last values from the PostgreSQL database.
 
Usage: `show-data [-c file]`

The configuration file path can be force to read a specific configuration. By default, the script read
`/etc/read-teleinfo.conf` and `.env` file.


### Dump data stored in database

The `dump` command export all data stored in the PostgreSQL database in CSV format.

Usage: `dump [-c file] output`

The configuration file path can be force to read a specific configuration. By default, the script read
`/etc/read-teleinfo.conf` and `.env` file. `output` is the output path.


## Test with the data generator

Run the data generator script to create file socket and send data into it:

    cargo run --bin data-generator
    
Run the reader script to read data from `/tmp/output`:

    cargo run --bin read-teleinfo /tmp/output -v
    
    
## Deploy executable by creating debian package that install a systemd service

First, install `cargo-deb`:

    cargo install cargo-deb
    
Build the debian package:

    cargo build --release && cargo deb
    
Install the debian package:

    sudo dpkg -i target/debian/teleinfo_reader_<version>_<arch>.deb

Configure the device path and database access by editing the `/etc/read-teleinfo.conf`.

Reload the daemon:

    sudo systemctl restart read-teleinfo
    
Check that the service is active:

    sudo systemctl status read-teleinfo
    sudo journalctl -u read-teleinfo  # to see the logs
    
Uninstall the package:

    sudo dpkg --purge read-teleinfo_reader
