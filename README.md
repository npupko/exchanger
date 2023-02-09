# Exchanger

Exchanger is a Rust-based command line utility that allows users to easily convert currencies. It retrieves currency rates from the National Bank of Belarus and calculates the exchange rate from US Dollars (USD) to Belarusian Rubles (BYN).

![Demo](/assets/demo.png)

## Installation

To install Exchanger, you will need to have the following installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Simply clone the repository and build the program using `cargo`:

```
git clone https://github.com/npupko/exchanger.git
cd exchanger
cargo build --release
```

## Usage

To use Exchanger, simply run the program and specify the date for rate to get

```
./exchanger --date <date>
```
You can also specify the the amount of USD you would like to convert to BYN:

```
./exchanger --date <date> --amount <amount>
./exchanger --date 2022-12-31 --amount 2000

# USD rate for 2022-12-31 is 2.7364
# You received (in BYN): 5472.8000
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
