# 0.7.0-rc.1

- Use base64 0.21.0-rc.1
- Support visibilities other than `pub`, e.g. `pub(crate)`

# 0.7.0-beta.2

- Update to base64 0.21.0-beta.2

# 0.7.0-beta.1

- Update to base64 0.21.0-beta.1
- Update to serde 1.0.152
- The `base64_serde_type` macro now uses an `Engine` instead of `Config` because that's how base64 0.20+ works.

# 0.6.1

- Use `base64` 0.13.0

# 0.6.0

- Now serializer works with any input type that implements `AsRef<[u8]>`
- Now deserializer works with any output type that implements `From<Vec<u8>>`

# 0.5.0

- Use `base64` 0.12.0
- Use `serde` (and related crates) 1.0.104 

# 0.4.0

- Use `base64` 0.11.0

# 0.3.1

- Bumped `base64` and `serde` versions

# 0.3.0

- Bumped the `base64` dependency version

# 0.2.0

- Add ability to make the generated type `pub`

# 0.1.1

- No functional changes, just better docs

# 0.1

- Initial release
