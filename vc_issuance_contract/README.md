[<img src="https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/a43a4a8b-932b-47e5-af63-470e35ab9330" width="300px" />][chaincerts.co]
<br/><br/>

# Verifiable Credentials Issuance Smart Contract
The Verifiable Credentials (VCs) Issuance smart contract establishes the rules for issuing, transferring, and revoking verifiable credentials. It acts as the governing framework, ensuring the secure and standardized management of on-chain verifiable credentials.

This smart contract prioritizes security and privacy by avoiding the persistence of structured data or personal information. Data is handled exclusively in an encrypted form, with access granted only to owners through cryptographic mechanisms. This approach is particularly critical in insecure communication channels such as blockchain ledgers. For more details, refer to the [W3C KeyAgreement specification](https://www.w3.org/TR/did-core/#dfn-keyagreement).

## Development

### Pre-requirements

Before getting started with the development of the Verifiable Credentials Issuance smart contract, ensure the following pre-requirements are met:

- [Soroban setup][soroban-setup].

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/soroban-did-contract.git
    ```

2. Build the project and install dependencies:
    ```
    cd soroban-did-contract
    soroban contract build
    ```

3. Run tests:
    ```
    cargo test
    ```

### Deployment

1. Build the contract:
    ```
    soroban contract build
    ```

    This will generate a WASM file for the contract in the `target/wasm32-unknown-unknown/release/` directory.

2. Deploy using Soroban CLI:
    ```bash
    soroban contract deploy \
        --source-account SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://rpc-futurenet.stellar.org \
        --network-passphrase 'Test SDF Network ; October 2022' \
        --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm

    CONTRACT_ID
    ```

## Vault Contract Functions

The following functions define the behavior of the VC issuance smart contract.

### `initialize`
Initializes the VC Issuance Contract by setting the admin.

```rust
fn initialize(e: Env, admin: Address);
```

#### Parameters:

- e: Environment object.
- admin: Address of the smart contract administrator.

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  initialize \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA 
```

### `issue`:
 Issues a new Verifiable Credential and returns the Verifiable Credential id as String. The admin account is the only party authorized to invoke this function.

```rust
fn issue(
    e: Env,
    admin: Address,
    recipient_did: String,
    vc_data: String,
    storage_address: Address,
) -> String;
```

#### Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `vc_data`: String representing encrypted Verifiable Credential data.
- `storage_address`: Vault smart contract address

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --storage_address GR2RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA
```

### `verify`
Verifies if the Verifiable Credential has been revoked, it returns a Map with the respective status.

```rust
fn verify(e: Env, vc_id: String) -> Map<String, String>;
```

#### Parameters:

- `e`: Environment object.
- `vc_id`: String representing the VC ID to verify.

#### Example: 

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  verify \
  --vc_id "vc_id"
```

### `revoke`
Revokes a verifiable credential in a specific date. The admin account is the only party authorized to invoke this function.

```rust
fn revoke(e: Env, admin: Address, vc_id: String, date: String);
```

#### Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `vc_id`: ID of the VC to be revoked.
- `date`: String representing the date where the VC is revoked.

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --vc_id "vc_id" \
  --date "01/01/2010 14:10:10"
```

## Deployment

1. Build the contract:
    ```
    soroban contract build
    ```

    This will generate a WASM file for the contract in the `target/wasm32-unknown-unknown/release/` directory.

2. Deploy using Soroban CLI:
    ```bash
    soroban contract deploy \
        --source-account SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://rpc-futurenet.stellar.org \
        --network-passphrase 'Test SDF Network ; October 2022' \
        --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm

    CONTRACT_ID
    ```

## Contract Errors

| Code | Error | Description |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | Contract has already been initialized |
| 2 | `NotAuthorized` | Invoker lacks the necessary authorization as the contract administrator |
| 3 | `AmountLimitExceeded` | The amount exceeds the issuance contract's capacity for certificates |
| 4 | `VCNotFound` | The Verifiable Credential (VC) was not found |

## Changelog
Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct
We welcome everyone to contribute. Make sure you have read the [CODE OF CONDUCT][coc] before.

## Contributing
For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License
This software is licensed under the [Apache License 2.0][license] © kommit.

<br/>

<hr/>

[<img src="https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/d60d775f-166b-4968-89b6-8be847993f8c" width="80px" alt="kommit"/>](https://kommit.co)

<sub>

[Website][kommit-website] •
[Github][kommit-github] •
[X][kommit-x] •
[LinkedIn][kommit-linkedin]

</sub>

[chaincerts.co]: https://chaincerts.co
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[license]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE
[coc]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CONTRIBUTING.md
[kommit-website]: https://kommit.co
[kommit-github]: https://github.com/kommitters
[kommit-x]: https://twitter.com/kommitco
[kommit-linkedin]: https://www.linkedin.com/company/kommit-co
