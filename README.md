# Substrate-project-template
Template, based on Parity Substrate, used to start new projects

## How to use template
This template is based on [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)
version 2.0.  To build the project it requires rust nightly-2020-10-05 and wasm32-unknown-unknown target.

### Create new custom pallet using pallet-template

1. set pallet `name` in pallet/template/Cargo.toml
   rename pallet/template folder into pallet/your_pallet_name

   edit dependencies in runtime/Cargo.toml accordingly

2. replace `TemplateModule` into your own module name

    pallet/template/src/lib.rs,
	    p.76
	pallet/template/src/mock.rs
	    p.109

	pallet/template/src/test.rs

	node/src/chain_spec.rs
	    p.3, p.165

	runtime/src/lib.rs
		p.295

	replace `pallet_template` in
	  node/src/chain_spec.rs p.165
	  runtime/src/lib.rs p.46, p.99, p.100, p.273, p.274, p.295

3. amend mock.rs  and tests.rs
    replace `template` module with your module name in mock.rs
	```rust
	mod template {
		pub use crate::Event;
	}
	...
	impl_outer_event! {
		pub enum Event for Test {
			system<T>,
			template<T>,
			balance<T>,
		}
	}
	```
	replace `Event::template` with  `Event::your_module` in tests.rs

## Test the pallet
   ```bash
   cargo test
   ```

## Implement your pallet,
   `pallet-template` implements basic account storage. Each account has
    role value - integer field that can be 1 (ADMIN), 2 (USER), 0 - disabled.
    Accounts with ADMIN role can create new one as well as disable other accounts.

    `pallet-template` unit tests have examples of using pallet-balances and pallet-timeout
     substrate runtime modules.

    Amend or replace lib.rs with your own code.
    Use provided [substrate documentation](https://substrate.dev/docs/en/knowledgebase/runtime/)

## Build node
   ```bash
   cargo build --release
   ```
## Generate documentation
   ```
   cargo doc
   ```

## Run single node
   ```bash
   ./target/release/node-template --dev
   ```
    Detailed logs may be shown by running the node with the following environment variables set:
    ```bash
    RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev
    ```
   Other CLI options available
   ```bash
   ./target/release/node-template -h
   ```
