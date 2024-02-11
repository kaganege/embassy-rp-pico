<div align="center">
  <h1>Embassy Raspberry Pi Pico Template</h1>
  <p><b>A template for kick starting a <a href="https://www.raspberrypi.com/products/raspberry-pi-pico/">Raspberry Pi Pico</a> project using <a href="https://embassy.dev/">Embassy</a>.</b></p>
</div>

## Usage

### Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```sh
cargo generate --git https://github.com/kaganege/embassy-rp-pico.git --name my-project
cd my-project
```

### Build and Upload

> [!WARNING]
> Your Pico must be in boot mode!

```sh
cargo run --release
```
