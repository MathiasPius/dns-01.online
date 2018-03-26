# dns-01.online

This is the mostly complete source code for dns-01.online.

## named/
Relevant configuration files for setting up the nameservers for dns-01.online, including some configuration options and setup for all the ns1-3 nameservers.

## nginx/
Nginx server configurations for the api.dns-01.online and dns-01.online domains.

## mysql/
Contains the SQL code for setting up the tables initially

## dns-01-web/
Rust project source code for the Rocket application behind the website.

## put-token.sh
Auth-hook script for certbot which reads API key and record name from environment variables and uses curl to push a token to the challenge.dns-01.online nameservers.


# Notes on running locally

## Database
Check mysql/ for the initial table setup scripts.

In order to connect to a database, the application uses the following environment variables to build a connection string: `$DNS01_HOSTNAME`, `$DNS01_USERNAME`. `$DNS01_PASSWORD`, `$DNS01_DATABASE`. If they are not set, the application will simply panic.

## Updating zone files
When a new record is pushed to the api, dns-01-web writes a new zone file based on the template pointed to by `$DNS01_ZONETEMPLATE`, as well as the records from the database. It then writes the resulting zone file to `$DNS01_ZONEFILE`, where `incrond` is listening for changes and calls `rndc reload` when it sees one.
