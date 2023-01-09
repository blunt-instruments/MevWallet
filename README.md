## MevWallet

MevWallet is a smart contract wallet that allows the user to capture MEV from
Searchers, or create MEV on purpose.

### Wait what?

MevWallet is a smart contract wallet that allows the user to capture MEV from
Searchers, or create MEV on purpose.

### Ok yeah I heard you the first time, what does that mean?

User transactions pay fees, and may generate MEV. Searchers take the MEV from
the user, and pay it to block proposers. This means that users are effectively
paying block creators twice. Once via MEV, and once via the regular transaction
fee.

MevWallet changes this relationship. Instead of paying block proposers via tx
fees, users make transactions via the MevWallet. These transaction pay no tx
fee. Instead, MevWallet transactions create MEV, and allow Searchers to pay tx
fees on the user's behalf. This allows users to more effectively and efficiently
price their transactions.

It also allows one more (**really cool** thing). It lets the user give the
Searcher _negative_ MEV. This means that the Searcher's bundle must _pay the
user_ for the right to broadcast the transaction. If the user is creating a
large amount of MEV, they can force the Searcher to give them a cut of that
MEV. The user gets to use this to cover their tx fees (cool!) and potentially
end up paying **less than 0** tx fees (coooler!).

### Addresses

- MevWeth: `0x00000000008C43efC014746c230049e330039Cb3`
- MevWalletV0 Implementation: `0x00000000007Dcbd85Fc67915ad4bE7DAE266e268`
- MevWalletV0 ProxyFactory: `0x385D9e104941e53fa73BBa3Ec3e9bAA4D1C5ad39`

Didn't bother to grind an address for the proxy factory :)

### How do I integrate it?

This repo has a Rust library for working with MevTxns. I'm also publishing a
typescript library using `ethers.js`.

Use `MevTxBuilder` to build a txn:

```rust
let mev_tx: MevTx = MevTxBuilder::new()
  .to(weth_address)
  .call(
    // WOW! you can use any ABI-gen generated call here!
    // (you can also you `.data(some_bytes)` to set the raw calldata
    ierc20::TransferCall {
      recipient,
      amount,
    }
  )
  .tip(1_000_000_000)
  .populate(&wallet_contract)
  .await?
  .build();
let signed_mev_tx = mev_tx.sign(wallet_address, signer).await?;

// You can convert from an ether-rs tx too!
// Middleware is required to resolve typed tx ENS name. This can often be a
// dummy middleware.
let typed_tx = weth.transfer(recipient, amount);
let signed_mev_tx = MevTxBuilder::from_typed_tx(middleware, typed_tx)
  .await?
  .nonce(5)
  // adding a signer signs the mev tx when it's built :)
  .with_signer(signer)
  // Defaults to the signer's chain_id
  .with_chain_id(31337)
  .populate()
  .await?
  .build(wallet_address)
  .await?;
```

Searchers should can use any middleware to send a `MevTx`:

```rust
let signed_mev_tx = serde_json::from_str(a_serialized_tx)?;

// `into_call` converts into a normal `ContractCall` wrapping the MevTx
// it sets value to the mev_tx_value, and
let pending = signed_mev_tx
  .into_call(arc_middleware.clone())
  // modify gas or w/e here :)
  // it is STRONGLY ADVISED that you use `.call()` to simulate
  // as we do not currently check the code at the wallet address
  .send()
  .await?;
```

You can also use this lib to deploy a new proxy contract:

```rust
// set owner to msg.sender
let my_wallet = MevWallet::new_proxy(middleware, salt).await?;

// set owner to an arbitrary address
let my_wallet = MevWallet::new_proxy(middleware, salt, owner_address).await?;
```

However, if doing it manually might be easier to use forge

```sh
forge script --broadcast $FORGE_SIGNER_INFO DeployProxyV0 --sig "run(bytes32)" $SALT -vvvvvv
```

### MevTx Layout

MevTxns are very similar to normal txns, and are signed with EIP712.

- `to` -- target address for the call.
- `data` -- data for the call.
- `value` -- value that should be passed in the call.
- `delegate` -- `true` for delegatecall (but be careful!). Must be false if
  value is non-0
- `tip` -- MEV created by the tx, in wei, can be negative
- `maxBaseFee` -- call should not be executed above this basefee (may be `0`,
  for no max)
- `deadline` -- call should not be executed after this timestamp (may be `0`,
  for no deadline)
- `nonce` -- nonce for the MevTx, as set in `MevWallet` state

### Repo Setup

`$ git submodule update --init --recursive`

### Running Tests

Integrations tests are a bit funky right now. Run `./devenv.sh` to start an
anvil instance. Then run `cargo test` in another terminal. Use ctrl+C to exist
the anvil instance when necessary.

### Regenerating contract bindings.

It is NOT RECOMMENDED that you make contract changes. If you must, make sure to
run `./bind.sh` to generate the rust bindings.
