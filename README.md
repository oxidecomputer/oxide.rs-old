# oxide.rs 

The Rust API client for Oxide.

- [Rust docs](https://docs.rs/oxide-api)
- [Oxide API Docs](https://docs.oxide.computer/api?lang=rust)

## Generating

You can trigger a build with the GitHub action to generate the client. This will
automatically update the client to the latest version based on the spec
at [spec.json](spec.json).

Alternatively, if you wish to generate the client locally, run:

```bash
$ make generate
```

## Contributing

Please do not change the code directly since it is generated. PRs that change
the code directly will be automatically closed by a bot.

