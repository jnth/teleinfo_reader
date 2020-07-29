# Teleinfo Reader

Rust program that read serial data from teleinformation flux.
Only work for the TEMPO subscription.
Send data into PostgreSQL database.


## Configuration

Create a `.env` file to configure the PostgreSQL connection:

 - `PG_HOST` (`localhost` by default),
 - `PG_PORT` (`5432` by default),
 - `PG_USER` (local user by default),
 - `PG_PASSWORD` (empty string by default),
 - `PG_DBNAME` (same as user name by default).
 
 
## CLI scripts

 - `read-teleinfo`: read the serial data and send them into a PostgreSQL database.
 - `data-generator`: open file socket and send fake serial data.
 
## Test with the data generator

Run the data generator script to create file socket and send data into it:

    cargo run --bin data-generator
    
Run the reader script to read data from `/tmp/output`:

    cargo run --bin read-teleinfo /tmp/output -v
    