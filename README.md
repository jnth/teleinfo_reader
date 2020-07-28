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
 
 
## CLI script

 - `read-teleinfo`: read the serial data and send them into a PostgreSQL database.
 