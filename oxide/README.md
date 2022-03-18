# `oxide-api`

A fully generated, opinionated API client library for Oxide.

[![docs.rs](https://docs.rs/oxide-api/badge.svg)](https://docs.rs/oxide-api)

## API Details

API for interacting with the Oxide control plane



### Contact


| url | email |
|----|----|
| <https://oxide.computer> | api@oxide.computer |



## Client Details

This client is generated from the [Oxide OpenAPI
specs](https://github.com/oxidecomputer/omicron) based on API spec version `0.0.1`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
oxide-api = "0.1.0-rc.18"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use oxide_api::Client;

let oxide = Client::new(
    String::from("api-key"),
    String::from("host"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `OXIDE_TOKEN`
- `OXIDE_HOST`

And then you can create a client from the environment.

```
use oxide_api::Client;

let oxide = Client::new_from_env();
```
