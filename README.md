# rust-test-crate

## Usage

```sh

```

## Notes

To preview the crate documentation
```sh
cargo doc --no-deps --open
```

To locally install all packages
```sh
cd rust-test-crate/
cargo build --release
```

To globally install the binary
```sh
cd rust-test-crate/
cargo install --path .
```

To run locally
```sh
./target/release/rust-test-crate --op [add,divide,multiply] --a [NUM] --b [NUM]
```

To run globally
```sh
rust-test-crate --op [add,divide,multiply] --a [NUM] --b [NUM]
```
