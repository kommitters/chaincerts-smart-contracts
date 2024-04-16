[![CHAINCERTS_LOGO](https://github.com/kommitters/chaincerts-smart-contracts/assets/39246879/5c7c3c50-f435-43ad-87e5-dad223eaa12a)][chaincerts.co]

# Verifiable Credential Issuance Smart Contract

The verifiable credential (VC) issuance smart contract establishes the rules for issuing, transferring, and revoking verifiable credentials. It acts as the governing framework, ensuring the secure and standardized management of on-chain verifiable credentials.

This smart contract prioritizes security and privacy by avoiding the persistence of structured data or personal information. Data is handled exclusively in an encrypted form, with access granted only to owners through cryptographic mechanisms. This approach is particularly critical in insecure communication channels such as blockchain ledgers. For more details, refer to the [W3C KeyAgreement specification](https://www.w3.org/TR/did-core/#dfn-keyagreement).

## Features
With this smart contract, you will be able to:

- Issue a verifiable credential.
- Verify a verifiable credential.
- Revoke a verifiable credential.
- Set the contract admin.
- Upgrade the contract.
- Get the contract version.

## Types

### Revocation
Represents a revoked verifiable credential.

### Attributes

| Name         | Type      | Values                                            |
| ------------ | --------- | ------------------------------------------------- |
| `vc_id` | `String` | The verifiable credential id.                      |
| `date`    | `String`    | The date of revocation. |

### Example

```bash
{
  "vc_id": "a4tkzct2njbbcdu2nfwr32ib",
  "date": "2023-12-05T21:37:44.389Z"
}
```

## Functions

### Initialize
Initializes the contract by setting the contract admin, the issuer DID and the limit amount of verifiable credentials that can be issued. The maximum amount allowed is **200**; if no amount is provided, the default value is **20**. An error will be triggered if the contract has already been initialized.

```rust
fn initialize(e: Env, admin: Address, issuer_did: String, amount: Option<u32>);
```

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  initialize \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --issuer_did "did:chaincerts:7dotwpyzo2weqj6oto6liic6"
```

### Issue
Issues a verifiable credential by making a cross-contract call to the Vault to store the VC, and returns the VC id. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:

- The issuance exceeds the predefined amount.
- Invoker is not the contract admin.

```rust
fn issue(
    e: Env,
    vc_id: String,
    vc_data: String,
    vault_contract: Address,
) -> String;
```

#### Output

Returns the verifiable credential id.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  issue \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib" \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --vault_contract CBRM3HA7GLEI6QQ3O55RUKVRDSQASARUPKK6NXKXKKPWEYLE533GDYQD

# Output: VC ID

"t5iwuct2njbbcdu2nfwr32ib"
```

### Verify
Verifies the verifiable credential status, returning a map indicating if it is **valid** or **revoked**. If the status is revoked, it additionally provides the date of revocation. An error will be triggered if the verifiable credential is not registered.

```rust
fn verify(e: Env, vc_id: String) -> Map<String, String>;
```

#### Output

Returns a map with the VC status.

###  Example - VC is valid

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  verify \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib"

# Output: VC Status
{
    "status": "valid"
}
```

###  Example - VC is revoked

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  verify \
  --vc_id "d2tqrct2njbbcdu2nfwr32ib"

# Output: VC Status
{
    "status": "revoked",
    "since": "2023-12-05T21:37:44.389Z"
}
```

### Revoke
Revokes a verifiable credential given its id and the date of revocation. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:

- Invoker is not the contract admin.
- Verifiable credential is not registered.
- Verifiable credential is already revoked.

```rust
fn revoke(e: Env, vc_id: String, date: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  revoke \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib" \
  --date "2023-12-05T21:37:44.389Z"
```

### Set contract admin
Replaces the current contract admin with a new one.

```rust
fn set_admin(e: Env, new_admin: Address);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  set_admin \
  --new_admin GCWZBFEKWUGQKYLCLI5ULI4DTXLEA7LPC5QVB55NZPC7FY2NGMLP4YMC

```

### Upgrade contract
Replaces the current contract code with a new one.

```rust
fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  upgrade \
  --new_wasm_hash 4e3e2a3e6286149775c308c8420fd87c9e5f655549073506f72b917577ef1e33

```

### Get contract version
Returns the contract version.

```rust
fn version(e: Env) -> String;
```

#### Output
Returns the contract version as a string.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  version

# Output: CONTRACT VERSION
"0.18.0"
```

## Contract Errors

| Code | Error                   | Description                                                             |
| ---- | ----------------------- | ----------------------------------------------------------------------- |
| 1    | `AlreadyInitialized`    | Contract has already been initialized                                   |
| 2    | `AmountLimitExceeded`   | Provided amount exceeds the maximum allowed                             |
| 3    | `VCNotFound`            | Verifiable credential not found                                         |
| 4    | `VCAlreadyRevoked`      | Verifiable credential already revoked                                   |
| 5    | `IssuanceLimitExceeded` | Contract issuance limit exceeded                                        |

## Development

### Pre-requirements

To develop and test the smart contract, you need to install Rust and the Soroban CLI. The process is outlined in the Soroban setup documentation, which can be accessed at [Soroban setup][soroban-setup].

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/chaincerts-smart-contracts.git
    ```

2. Build the project and install dependencies:
    ```
    cd chaincerts-smart-contracts
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
    ```
    soroban contract deploy \
        --source SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://soroban-testnet.stellar.org:443 \
        --network-passphrase 'Test SDF Network ; September 2015' \
        --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm

    CONTRACT_ID
    ```

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
[stellar]: https://stellar.org
