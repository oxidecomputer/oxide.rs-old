> **Warning**
>
> This API client is no longer supported and will not be maintained.
> Refer to https://github.com/oxidecomputer/oxide-sdk-and-cli for the current CLI and Rust client

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

**Important: Currently the spec is being [modified](./scripts/tag-swap.sh) to use the old operation IDs as a lot of work needs to be done with the current generator.**

## Contributing

Please do not change the code directly since it is generated. PRs that change
the code directly will be automatically closed by a bot.

### Releasing a new version

1. Make sure the `VERSION.txt` has the new version you want to release.
2. Make sure you have run `make generate` and pushed any changes. The release
   will fail if running `make generate` causes any changes to the generated
   code.
3. Run `make tag` this is just an easy command for making a tag formatted
   correctly with the version.
4. Push the tag (the result of `make tag` gives instructions for this)
5. Everything else is triggered from the tag push. Just make sure all the tests
   pass on the `main` branch before making and pushing a new tag.
