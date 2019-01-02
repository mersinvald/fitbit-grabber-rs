# fitbit-grabber

This is a small utility for fetching data from the FitBit web API.

[![Build Status](https://travis-ci.org/bradfordboyle/fitbit-grabber-rs.svg?branch=master)](https://travis-ci.org/bradfordboyle/fitbit-grabber-rs)

It currently supports:

- Heart Rate
- Steps
- User Profile
- Sleep

## Building

```sh
cargo build [--release] --bin fitbit-grabber
```

## Using

Run `fitbit-grabber help` for a list of available subcommands.

To use `fitbit-grabber`, you will need to have an

1. client ID
2. client secret

To obtain a client ID and secret, [register an application][].

1. For "OAuth 2.0 Application Type", select "Personal". The reason for this
   requirement is that intraday series data is only available for persoanl
   applications.
2. For "Callback URL", enter "http://127.0.0.1:8080".

After completing the registration, you will need to export the client id and
secret as environmner variables:

```sh
export FITBIT_CLIENT_ID=<client-id>
export FITBIT_CLIENT_SECRET=<client-secret>
```

### Requesting an OAuth 2.0 Token

You will need to first generate and store a token for subsequent API calls

```sh
./fitbit-grabber token
```

If the above command is successful, the OAuth 2.0 token will be stored in a file
called ".token" in the working directory.

[register an application]: https://dev.fitbit.com/apps/new
