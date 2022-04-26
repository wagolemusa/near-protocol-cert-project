Event Booking in Rust
=================================

[![Open in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-counter)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

## Description

This contract implements simple Event Booking application, users have to checkIn for the event by stacking same near.
When a user checkIn and He did'not show up to the event, His near will not be returned to there wallet.


## To Run
Open in the Gitpod link above or clone the repository.

```
git clone https://github.com/wagolemusa/
```


## Setup [Or skip to Login if in Gitpod](#login)
Install dependencies:

```
yarn
```

If you don't have `Rust` installed, complete the following 3 steps:

1) Install Rustup by running:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Taken from official installation guide](https://www.rust-lang.org/tools/install))

2) Configure your current shell by running:

```
source $HOME/.cargo/env
```

3) Add wasm target to your toolchain by running:

```
rustup target add wasm32-unknown-unknown
```

Next, make sure you have `near-cli` by running:

```
near --version
```

If you need to install `near-cli`:

```
npm install near-cli -g
```

## Login
If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Modify the top of `src/config.js`, changing the `CONTRACT_NAME` to be the NEAR account that was just used to log in.

```javascript
…
const CONTRACT_NAME = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
…
```

Start the example!

```
yarn start
```

## To Test

```
cd src
cargo test -- --nocapture
```

## To Explore

- `src/lib.rs` for the contract code


## To Build the Documentation

```
cd contract
cargo doc --no-deps --open
```


# Required Software
- Rust 1.58 + cargo
- Node.js
- NEAR CLI 3.1

# Authors

- Wagole musa <wagolemusa@gmail.com> [@refugewize](https://twitter.com/refugewize)
