# Teleinfo Reader

Rust program that read serial data from teleinformation flux.
Only work for the TEMPO subscription.
Send data into PostgreSQL database.


## Configuration

Create a `.env` file to configure the PostgreSQL connection with the variable `DATABASE_URL` (like 
`postgres://user:password@database-host:port/dbname`, see `diesel` documentation for more information).

 
## CLI scripts

 - `read-teleinfo`: read the serial data and send them into a PostgreSQL database.
 - `data-generator`: open file socket and send fake serial data.
 - `show-data`: read and show the last values from the PostgreSQL database.
 
## Test with the data generator

Run the data generator script to create file socket and send data into it:

    cargo run --bin data-generator
    
Run the reader script to read data from `/tmp/output`:

    cargo run --bin read-teleinfo /tmp/output -v
    