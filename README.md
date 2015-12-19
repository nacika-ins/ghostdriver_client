# ghostdriver-client

Client of ghostdriver implementation of phantomjs

## Dependencies
* phantomjs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.ghostdriver_client]
git = "https://github.com/nacika-ins/ghostdriver_client.git"
```

Run PhantomJS

```bash
phantomjs --ignore-ssl-errors=yes --webdriver=8910
```

## Example

Please look at the [example.rs](https://github.com/nacika-ins/ghostdriver_client/blob/master/examples/sample.rs)